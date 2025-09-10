use super::{BlobMetadata, Storage, StorageConfig};
use anyhow::Result;
use async_trait:        match self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
        {
            Ok(response) => {
                let data = response.body.collect().await?.into_bytes();
                Ok(Some(data))
            }
            Err(SdkError::ServiceError(err)) if err.err().is_no_such_key() => Ok(None),
            Err(err) => Err(err.into()),
        }se aws_sdk_s3::config::{BehaviorVersion, Credentials};
use aws_config::Region;
use aws_sdk_s3::{Client as S3Client, primitives::ByteStream};
use aws_sdk_s3::error::SdkError;
use bytes::Bytes;
use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncWrite};

pub struct S3Storage {
    client: S3Client,
    bucket: String,
}

pub struct S3Config {
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub use_path_style: bool,
}

impl S3Storage {
    pub async fn new(config: &S3Config) -> Result<Self> {
        let creds = Credentials::new(
            &config.access_key_id,
            &config.secret_access_key,
            None,
            None,
            "s3-storage",
        );

        let region = Region::new(config.region.clone());

        let s3_config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(region)
            .endpoint_url(&config.endpoint)
            .credentials_provider(creds)
            .force_path_style(config.use_path_style)
            .build();

        let client = S3Client::from_conf(s3_config);

        Ok(Self {
            client,
            bucket: config.bucket.clone(),
        })
    }
}

#[async_trait]
impl Storage for S3Storage {
    async fn put_blob(&self, digest: &str, data: Bytes) -> Result<()> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(digest)
            .body(data.into())
            .send()
            .await?;
        Ok(())
    }

    async fn put_blob_streaming(
        &self,
        digest: &str,
        content_length: u64,
        data: Box<dyn AsyncRead + Send + Unpin>,
    ) -> Result<()> {
        let stream = tokio_util::io::ReaderStream::new(data);

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(digest)
            .content_length(content_length as i64)
            .body(ByteStream::new(stream))
            .send()
            .await?;
        Ok(())
    }

    async fn get_blob(&self, digest: &str) -> Result<Option<Bytes>> {
        match self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(digest)
            .send()
            .await
        {
            Ok(response) => {
                let data = response.body.collect().await?.into_bytes();
                Ok(Some(data))
            }
            Err(SdkError::ServiceError(err)) if err.err().is_no_such_key() => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    async fn get_blob_streaming(
        &self,
        digest: &str,
    ) -> Result<Option<Box<dyn AsyncRead + Send + Unpin>>> {
        match self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(digest)
            .send()
            .await
        {
            Ok(response) => {
                let stream = response.body;
                Ok(Some(Box::new(stream.into_async_read())))
            }
            Err(SdkError::ServiceError(err)) if err.err().is_no_such_key() => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    async fn delete_blob(&self, digest: &str) -> Result<bool> {
        match self
            .client
            .delete_object()
            .bucket(&self.bucket)
            .key(digest)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(SdkError::ServiceError(err)) if err.err().is_no_such_key() => Ok(false),
            Err(err) => Err(err.into()),
        }
    }

    async fn blob_exists(&self, digest: &str) -> Result<bool> {
        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(digest)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(SdkError::ServiceError(err)) if err.err().is_no_such_key() => Ok(false),
            Err(err) => Err(err.into()),
        }
    }

    async fn get_blob_metadata(&self, digest: &str) -> Result<Option<BlobMetadata>> {
        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(digest)
            .send()
            .await
        {
            Ok(response) => Ok(Some(BlobMetadata {
                size: response.content_length.unwrap_or(0) as u64,
                digest: digest.to_string(),
                created_at: response.last_modified.unwrap().to_chrono_utc().unwrap(),
                content_type: response.content_type,
            })),
            Err(SdkError::ServiceError(err)) if err.err().is_no_such_key() => Ok(None),
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
        let storage = tokio::runtime::Handle::current().block_on(S3Storage::new(self))?;
        Ok(Box::new(storage))
    }
}
