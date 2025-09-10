use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use chrono::Utc;
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncRead;

use aerugo::storage::{BlobMetadata, Storage};

#[derive(Default, Clone)]
pub struct MockStorage {
    blobs: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    metadata: Arc<Mutex<HashMap<String, BlobMetadata>>>,
}

impl MockStorage {
    pub fn new() -> Self {
        Self {
            blobs: Arc::new(Mutex::new(HashMap::new())),
            metadata: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Storage for MockStorage {
    async fn put_blob(&self, digest: &str, data: Bytes) -> Result<()> {
        let mut blobs = self.blobs.lock().unwrap();
        let mut metadata = self.metadata.lock().unwrap();

        blobs.insert(digest.to_string(), data.to_vec());
        metadata.insert(
            digest.to_string(),
            BlobMetadata {
                size: data.len() as u64,
                digest: digest.to_string(),
                created_at: Utc::now(),
                content_type: None,
            },
        );

        Ok(())
    }

    async fn put_blob_streaming(
        &self,
        digest: &str,
        content_length: u64,
        mut data: Box<dyn AsyncRead + Send + Unpin>,
    ) -> Result<()> {
        use tokio::io::AsyncReadExt;

        let mut buffer = Vec::new();
        data.read_to_end(&mut buffer).await?;

        let mut blobs = self.blobs.lock().unwrap();
        let mut metadata = self.metadata.lock().unwrap();

        blobs.insert(digest.to_string(), buffer.clone());
        metadata.insert(
            digest.to_string(),
            BlobMetadata {
                size: buffer.len() as u64,
                digest: digest.to_string(),
                created_at: Utc::now(),
                content_type: None,
            },
        );

        Ok(())
    }

    async fn get_blob(&self, digest: &str) -> Result<Option<Bytes>> {
        let blobs = self.blobs.lock().unwrap();
        Ok(blobs.get(digest).map(|data| Bytes::from(data.clone())))
    }

    async fn get_blob_streaming(
        &self,
        digest: &str,
    ) -> Result<Option<Box<dyn AsyncRead + Send + Unpin>>> {
        let blobs = self.blobs.lock().unwrap();
        Ok(blobs
            .get(digest)
            .map(|data| Box::new(Cursor::new(data.clone())) as Box<dyn AsyncRead + Send + Unpin>))
    }

    async fn delete_blob(&self, digest: &str) -> Result<bool> {
        let mut blobs = self.blobs.lock().unwrap();
        let mut metadata = self.metadata.lock().unwrap();

        let existed = blobs.remove(digest).is_some();
        metadata.remove(digest);

        Ok(existed)
    }

    async fn blob_exists(&self, digest: &str) -> Result<bool> {
        let blobs = self.blobs.lock().unwrap();
        Ok(blobs.contains_key(digest))
    }

    async fn get_blob_metadata(&self, digest: &str) -> Result<Option<BlobMetadata>> {
        let metadata = self.metadata.lock().unwrap();
        Ok(metadata.get(digest).cloned())
    }

    async fn health_check(&self) -> Result<()> {
        // Mock storage is always healthy
        Ok(())
    }
}
