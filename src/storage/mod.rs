use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncWrite};

/// Metadata about a stored blob
#[derive(Debug, Clone)]
pub struct BlobMetadata {
    pub size: u64,
    pub digest: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub content_type: Option<String>,
}

/// Storage backend trait that must be implemented by all storage providers
#[async_trait]
pub trait Storage: Send + Sync + 'static {
    /// Store a blob with the given digest
    async fn put_blob(&self, digest: &str, data: Bytes) -> Result<()>;

    /// Store a blob from a stream
    async fn put_blob_streaming(
        &self,
        digest: &str,
        content_length: u64,
        data: Box<dyn AsyncRead + Send + Unpin>,
    ) -> Result<()>;

    /// Get a blob by its digest
    async fn get_blob(&self, digest: &str) -> Result<Option<Bytes>>;

    /// Get a blob as a stream
    async fn get_blob_streaming(
        &self,
        digest: &str,
    ) -> Result<Option<Box<dyn AsyncRead + Send + Unpin>>>;

    /// Delete a blob by its digest
    async fn delete_blob(&self, digest: &str) -> Result<bool>;

    /// Check if a blob exists
    async fn blob_exists(&self, digest: &str) -> Result<bool>;

    /// Get metadata about a blob
    async fn get_blob_metadata(&self, digest: &str) -> Result<Option<BlobMetadata>>;

    /// Perform a health check on the storage backend
    async fn health_check(&self) -> Result<()>;
}

/// Storage configuration trait that must be implemented by all storage providers
pub trait StorageConfig: Send + Sync + 'static {
    /// Create a new storage backend from this configuration
    fn create_storage(&self) -> Result<Box<dyn Storage>>;
}

// Re-export storage implementations
pub mod filesystem;
pub mod s3;
