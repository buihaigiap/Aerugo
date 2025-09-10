use super::{BlobMetadata, Storage, StorageConfig};
use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncRead, AsyncWrite};

pub struct FilesystemStorage {
    root_path: PathBuf,
}

pub struct FilesystemConfig {
    pub root_path: PathBuf,
}

impl FilesystemStorage {
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    fn blob_path(&self, digest: &str) -> PathBuf {
        // Create a directory structure based on the digest to avoid too many files in one directory
        let dir1 = &digest[0..2];
        let dir2 = &digest[2..4];
        self.root_path.join(dir1).join(dir2).join(digest)
    }
}

#[async_trait]
impl Storage for FilesystemStorage {
    async fn put_blob(&self, digest: &str, data: Bytes) -> Result<()> {
        let path = self.blob_path(digest);

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(path, data).await?;
        Ok(())
    }

    async fn put_blob_streaming(
        &self,
        digest: &str,
        content_length: u64,
        mut data: Box<dyn AsyncRead + Send + Unpin>,
    ) -> Result<()> {
        let path = self.blob_path(digest);

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let mut file = fs::File::create(path).await?;
        tokio::io::copy(&mut data, &mut file).await?;
        Ok(())
    }

    async fn get_blob(&self, digest: &str) -> Result<Option<Bytes>> {
        let path = self.blob_path(digest);
        match fs::read(path).await {
            Ok(data) => Ok(Some(Bytes::from(data))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn get_blob_streaming(
        &self,
        digest: &str,
    ) -> Result<Option<Box<dyn AsyncRead + Send + Unpin>>> {
        let path = self.blob_path(digest);
        match fs::File::open(path).await {
            Ok(file) => Ok(Some(Box::new(file))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn delete_blob(&self, digest: &str) -> Result<bool> {
        let path = self.blob_path(digest);
        match fs::remove_file(path).await {
            Ok(()) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    async fn blob_exists(&self, digest: &str) -> Result<bool> {
        let path = self.blob_path(digest);
        Ok(path.exists())
    }

    async fn get_blob_metadata(&self, digest: &str) -> Result<Option<BlobMetadata>> {
        let path = self.blob_path(digest);
        match fs::metadata(path).await {
            Ok(metadata) => Ok(Some(BlobMetadata {
                size: metadata.len(),
                digest: digest.to_string(),
                created_at: chrono::DateTime::from(metadata.created()?),
                content_type: None,
            })),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn health_check(&self) -> Result<()> {
        // Check if root directory exists and is writable
        if !self.root_path.exists() {
            fs::create_dir_all(&self.root_path).await?;
        }

        // Try to write and read a test file
        let test_path = self.root_path.join(".health_check");
        fs::write(&test_path, b"health check").await?;
        fs::remove_file(test_path).await?;

        Ok(())
    }
}

impl StorageConfig for FilesystemConfig {
    fn create_storage(&self) -> Result<Box<dyn Storage>> {
        Ok(Box::new(FilesystemStorage::new(self.root_path.clone())))
    }
}
