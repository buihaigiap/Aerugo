use super::{BlobMetadata, Storage, StorageConfig};
use anyhow::{Context, Result};
use async_trait::async_trait;
use aws_config::{retry::RetryConfig, Region};
use aws_sdk_s3::config::{Builder as S3ConfigBuilder, Credentials};
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::{primitives::ByteStream, Client as S3Client};
use bytes::Bytes;
use futures::StreamExt;
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::io::ReaderStream;
use tracing::{error, warn};

pub struct S3Storage {
    client: S3Client,
    bucket: String,
    multipart_threshold: u64,
    part_size: u64,
}

impl S3Storage {
    fn make_key(&self, key: &str) -> String {
        // Return the key as-is, allowing callers to specify full path structure
        key.to_string()
    }
}

#[derive(Error, Debug)]
pub enum S3StorageError {
    #[error("Failed to upload object: {0}")]
    UploadError(String),
    #[error("Failed to download object: {0}")]
    DownloadError(String),
    #[error("Failed to delete object: {0}")]
    DeleteError(String),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub struct S3Config {
    pub endpoint: String,
    pub bucket: String,
    pub region: String,
    pub auth_method: S3AuthMethod,
    pub use_path_style: bool,
    pub retry_attempts: Option<u32>,
    pub multipart_threshold: Option<u64>,
    pub part_size: Option<u64>,
}

#[derive(Clone, Debug)]
pub enum S3AuthMethod {
    Static {
        access_key_id: String,
        secret_access_key: String,
    },
    AssumeRole {
        role_arn: String,
        external_id: Option<String>,
    },
    WebIdentity {
        role_arn: String,
        token_file: String,
    },
    Environment,
}

impl S3Storage {
    pub async fn new(config: &S3Config) -> Result<Self> {
        let region = Region::new(config.region.clone());

        // Set up credentials
        let credentials = match &config.auth_method {
            S3AuthMethod::Static {
                access_key_id,
                secret_access_key,
            } => Credentials::new(
                access_key_id,
                secret_access_key,
                None,
                None,
                "static-provider",
            ),
            S3AuthMethod::AssumeRole { .. } => {
                return Err(anyhow::anyhow!("AssumeRole authentication not implemented"))
            }
            S3AuthMethod::WebIdentity { .. } => {
                return Err(anyhow::anyhow!(
                    "WebIdentity authentication not implemented"
                ))
            }
            S3AuthMethod::Environment => {
                return Err(anyhow::anyhow!(
                    "Environment authentication not implemented"
                ))
            }
        };

        // Create retry config
        let retry_config =
            RetryConfig::standard().with_max_attempts(config.retry_attempts.unwrap_or(3));

        // Create S3 configuration
        let s3_config = S3ConfigBuilder::new()
            .endpoint_url(&config.endpoint)
            .region(Some(region))
            .force_path_style(config.use_path_style)
            .behavior_version_latest()
            .retry_config(retry_config)
            .credentials_provider(credentials)
            .build();

        // Create S3 client
        let client = S3Client::from_conf(s3_config);

        Ok(Self {
            client,
            bucket: config.bucket.clone(),
            multipart_threshold: config.multipart_threshold.unwrap_or(100 * 1024 * 1024),
            part_size: config.part_size.unwrap_or(10 * 1024 * 1024),
        })
    }

    async fn handle_error<T>(&self, result: Result<T>, context: &str) -> Result<T> {
        match result {
            Ok(value) => Ok(value),
            Err(err) => {
                error!(?err, %context, "S3 operation failed");
                Err(anyhow::anyhow!("{}: {}", context, err))
            }
        }
    }

    async fn abort_multipart_upload(&self, key: &str, upload_id: &str) -> Result<()> {
        match self
            .client
            .abort_multipart_upload()
            .bucket(&self.bucket)
            .key(key)
            .upload_id(upload_id)
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                warn!(
                    ?err,
                    key,
                    upload_id,
                    "Failed to abort multipart upload, this may need manual cleanup"
                );
                Ok(()) // Don't fail the operation if cleanup fails
            }
        }
    }
}

#[async_trait]
impl Storage for S3Storage {
    async fn put_blob(&self, key: &str, data: Bytes) -> Result<()> {
        let storage_key = self.make_key(key);
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&storage_key)
            .body(ByteStream::from(data))
            .send()
            .await?;
        Ok(())
    }

    async fn put_blob_streaming(
        &self,
        key: &str,
        content_length: u64,
        data: Box<dyn AsyncRead + Send + Unpin>,
    ) -> Result<()> {
        if content_length < self.multipart_threshold {
            // For small files, use simple upload
            let stream = ReaderStream::new(data);
            let mut bytes = Vec::with_capacity(content_length as usize);
            tokio::pin!(stream);
            while let Some(chunk) = stream.next().await {
                let chunk = chunk.context("Failed to read from stream")?;
                bytes.extend_from_slice(&chunk);
            }
            let body = ByteStream::from(bytes);

            let storage_key = self.make_key(key);
            self.client
                .put_object()
                .bucket(&self.bucket)
                .key(&storage_key)
                .content_length(content_length as i64)
                .body(body)
                .send()
                .await
                .context("Failed to upload small blob")?;
            return Ok(());
        }

        // For large files, use multipart upload
        let storage_key = self.make_key(key);
        let multipart = self
            .client
            .create_multipart_upload()
            .bucket(&self.bucket)
            .key(&storage_key)
            .send()
            .await
            .context("Failed to initiate multipart upload")?;

        let stream = ReaderStream::new(data);
        let mut stream = Box::pin(stream);
        let mut part_number = 1;
        let mut upload_parts = Vec::new();
        let mut buffer = Vec::with_capacity(self.part_size as usize);

        // Upload parts
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.context("Failed to read chunk from stream")?;
            buffer.extend_from_slice(&chunk);

            if buffer.len() >= self.part_size as usize {
                let part_data = std::mem::take(&mut buffer);
                let upload_part_result = self
                    .client
                    .upload_part()
                    .bucket(&self.bucket)
                    .key(&storage_key)
                    .upload_id(multipart.upload_id().unwrap())
                    .part_number(part_number)
                    .body(ByteStream::from(part_data))
                    .send()
                    .await
                    .context("Failed to upload part")?;

                upload_parts.push(
                    aws_sdk_s3::types::CompletedPart::builder()
                        .e_tag(upload_part_result.e_tag.unwrap())
                        .part_number(part_number)
                        .build(),
                );

                part_number += 1;
            }
        }

        // Upload the last part if there's any data left in the buffer
        if !buffer.is_empty() {
            let part_data = std::mem::take(&mut buffer);
            let upload_part_result = self
                .client
                .upload_part()
                .bucket(&self.bucket)
                .key(&storage_key)
                .upload_id(multipart.upload_id().unwrap())
                .part_number(part_number)
                .body(ByteStream::from(part_data))
                .send()
                .await
                .context("Failed to upload final part")?;

            upload_parts.push(
                aws_sdk_s3::types::CompletedPart::builder()
                    .e_tag(upload_part_result.e_tag.unwrap())
                    .part_number(part_number)
                    .build(),
            );
        }

        // Complete multipart upload
        self.client
            .complete_multipart_upload()
            .bucket(&self.bucket)
            .key(&storage_key)
            .upload_id(multipart.upload_id().unwrap())
            .multipart_upload(
                aws_sdk_s3::types::CompletedMultipartUpload::builder()
                    .set_parts(Some(upload_parts))
                    .build(),
            )
            .send()
            .await
            .context("Failed to complete multipart upload")?;

        Ok(())
    }

    async fn get_blob(&self, key: &str) -> Result<Option<Bytes>> {
        let storage_key = self.make_key(key);
        match self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(&storage_key)
            .send()
            .await
        {
            Ok(response) => {
                let data = response.body.collect().await?.into_bytes();
                Ok(Some(data))
            }
            Err(SdkError::ServiceError(_)) => Ok(None), // Assume not found for any service error
            Err(err) => Err(err.into()),
        }
    }

    async fn get_blob_streaming(
        &self,
        key: &str,
    ) -> Result<Option<Box<dyn AsyncRead + Send + Unpin>>> {
        let storage_key = self.make_key(key);
        match self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(&storage_key)
            .send()
            .await
        {
            Ok(response) => {
                let stream = response.body;
                Ok(Some(Box::new(stream.into_async_read())))
            }
            Err(SdkError::ServiceError(_)) => Ok(None), // Assume not found for any service error
            Err(err) => Err(err.into()),
        }
    }

    async fn delete_blob(&self, key: &str) -> Result<bool> {
        let storage_key = self.make_key(key);
        match self
            .client
            .delete_object()
            .bucket(&self.bucket)
            .key(&storage_key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(SdkError::ServiceError(_)) => Ok(false), // Assume not found for any service error
            Err(err) => Err(err.into()),
        }
    }

    async fn blob_exists(&self, key: &str) -> Result<bool> {
        let storage_key = self.make_key(key);
        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(&storage_key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(SdkError::ServiceError(_)) => Ok(false), // Assume not found for any service error
            Err(err) => Err(err.into()),
        }
    }

    async fn get_blob_metadata(&self, key: &str) -> Result<Option<BlobMetadata>> {
        let storage_key = self.make_key(key);
        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(&storage_key)
            .send()
            .await
        {
            Ok(response) => {
                let last_modified = response.last_modified.unwrap();
                let secs = last_modified.as_secs_f64() as i64;
                let nanos = ((last_modified.as_secs_f64().fract() * 1_000_000_000.0) as u32)
                    .min(999_999_999);

                Ok(Some(BlobMetadata {
                    size: response.content_length.unwrap_or(0) as u64,
                    digest: key.to_string(),
                    created_at: chrono::DateTime::from_timestamp(secs, nanos)
                        .unwrap_or_else(|| chrono::Utc::now()),
                    content_type: response.content_type,
                }))
            }
            Err(SdkError::ServiceError(_)) => Ok(None), // Assume not found for any service error
            Err(err) => Err(err.into()),
        }
    }

    async fn health_check(&self) -> Result<()> {
        // Try to list objects to verify connectivity and permissions
        self.client
            .list_objects_v2()
            .bucket(&self.bucket)
            .max_keys(1)
            .send()
            .await?;
        Ok(())
    }
}

impl StorageConfig for S3Config {
    fn create_storage(&self) -> Result<Box<dyn Storage>> {
        // Use tokio runtime to initialize the storage
        let runtime = tokio::runtime::Handle::current();
        let storage = runtime.block_on(S3Storage::new(self))?;
        Ok(Box::new(storage))
    }
}
