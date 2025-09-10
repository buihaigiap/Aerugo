mod utils;

use aerugo::storage::Storage;
use anyhow::Result;
use bytes::Bytes;
use tokio::io::AsyncReadExt;
use utils::MockStorage;

#[tokio::test]
async fn test_basic_blob_operations() -> Result<()> {
    let storage = MockStorage::new();
    let test_digest = "sha256:1234567890abcdef";
    let test_content = Bytes::from("Hello, World!");

    // Test put_blob
    storage.put_blob(test_digest, test_content.clone()).await?;

    // Test blob_exists
    assert!(storage.blob_exists(test_digest).await?);
    assert!(!storage.blob_exists("nonexistent").await?);

    // Test get_blob
    if let Some(data) = storage.get_blob(test_digest).await? {
        assert_eq!(data, test_content);
    } else {
        panic!("Blob not found");
    }

    // Test get_blob for nonexistent path
    assert!(storage.get_blob("nonexistent").await?.is_none());

    // Test get_blob_metadata
    if let Some(metadata) = storage.get_blob_metadata(test_digest).await? {
        assert_eq!(metadata.size, test_content.len() as u64);
        assert_eq!(metadata.digest, test_digest);
    } else {
        panic!("Metadata not found");
    }

    // Test delete_blob
    assert!(storage.delete_blob(test_digest).await?);
    assert!(!storage.blob_exists(test_digest).await?);
    assert!(storage.get_blob_metadata(test_digest).await?.is_none());

    Ok(())
}

#[tokio::test]
async fn test_streaming_operations() -> Result<()> {
    let storage = MockStorage::new();
    let test_digest = "sha256:streaming1234";
    let test_content = Bytes::from("Hello from stream!");

    // Test put_blob_streaming
    {
        let content_length = test_content.len() as u64;
        let reader = Box::new(std::io::Cursor::new(test_content.clone()));
        storage
            .put_blob_streaming(test_digest, content_length, reader)
            .await?;
    }

    // Test get_blob_streaming
    if let Some(mut reader) = storage.get_blob_streaming(test_digest).await? {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        assert_eq!(buf, test_content);
    } else {
        panic!("Streaming blob not found");
    }

    Ok(())
}

#[tokio::test]
async fn test_concurrent_access() -> Result<()> {
    use futures::future::join_all;
    use tokio::spawn;

    let storage = MockStorage::new();
    let num_operations = 10;

    // Concurrent writes
    let writes = (0..num_operations)
        .map(|i| {
            let storage = storage.clone();
            let digest = format!("sha256:concurrent{}", i);
            let content = Bytes::from(vec![i as u8]);
            spawn(async move { storage.put_blob(&digest, content).await })
        })
        .collect::<Vec<_>>();

    // Wait for all writes to complete
    for result in join_all(writes).await {
        result??;
    }

    // Verify all writes
    for i in 0..num_operations {
        let digest = format!("sha256:concurrent{}", i);
        if let Some(data) = storage.get_blob(&digest).await? {
            assert_eq!(data, Bytes::from(vec![i as u8]));
        } else {
            panic!("Blob not found: {}", digest);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_error_conditions() -> Result<()> {
    let storage = MockStorage::new();
    let test_digest = "sha256:nonexistent";

    // Test getting nonexistent blob
    assert!(storage.get_blob(test_digest).await?.is_none());
    assert!(storage.get_blob_streaming(test_digest).await?.is_none());
    assert!(storage.get_blob_metadata(test_digest).await?.is_none());

    // Test deleting nonexistent blob
    assert!(!storage.delete_blob(test_digest).await?);

    Ok(())
}

#[tokio::test]
async fn test_health_check() -> Result<()> {
    let storage = MockStorage::new();
    storage.health_check().await?;
    Ok(())
}
