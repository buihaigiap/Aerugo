use aerugo::storage::{
    s3::{S3AuthMethod, S3Config, S3Storage},
    Storage,
};
use bytes::Bytes;
use std::time::Duration;
use tokio::io::AsyncReadExt;

#[tokio::test]
async fn test_s3_basic_operations() {
    let config = setup_test_config();
    let storage = S3Storage::new(&config)
        .await
        .expect("Failed to create S3 storage");

    // Test data
    let test_data = b"Hello, this is a test file!".to_vec();
    let digest = "test-file-1";

    // Test put_blob
    storage
        .put_blob(digest, Bytes::from(test_data.clone()))
        .await
        .expect("Failed to put blob");

    // Test blob_exists
    assert!(storage
        .blob_exists(digest)
        .await
        .expect("Failed to check blob existence"));

    // Test get_blob
    let retrieved_data = storage
        .get_blob(digest)
        .await
        .expect("Failed to get blob")
        .expect("Blob not found");
    assert_eq!(retrieved_data.to_vec(), test_data);

    // Test delete_blob
    assert!(storage
        .delete_blob(digest)
        .await
        .expect("Failed to delete blob"));
    assert!(!storage
        .blob_exists(digest)
        .await
        .expect("Failed to check blob existence after deletion"));
}

#[tokio::test]
async fn test_multipart_upload() {
    let config = setup_test_config();
    let storage = S3Storage::new(&config)
        .await
        .expect("Failed to create S3 storage");

    // Create a large file (15MB)
    let large_data = vec![0u8; 15 * 1024 * 1024];
    let digest = "test-large-file";

    // Test streaming upload
    let reader = std::io::Cursor::new(large_data.clone());
    storage
        .put_blob_streaming(digest, large_data.len() as u64, Box::new(reader))
        .await
        .expect("Failed to upload large file");

    // Verify upload
    let retrieved_data = storage
        .get_blob_streaming(digest)
        .await
        .expect("Failed to get blob stream")
        .expect("Blob stream not found");

    let mut buffer = Vec::new();
    let mut reader = retrieved_data;
    reader
        .read_to_end(&mut buffer)
        .await
        .expect("Failed to read stream");

    assert_eq!(buffer.len(), large_data.len());
    assert_eq!(buffer, large_data);

    // Cleanup
    storage
        .delete_blob(digest)
        .await
        .expect("Failed to delete large blob");
}

#[tokio::test]
async fn test_error_handling() {
    let mut config = setup_test_config();

    // Test with invalid credentials
    config.auth_method = S3AuthMethod::Static {
        access_key_id: "invalid".to_string(),
        secret_access_key: "invalid".to_string(),
    };

    let storage = S3Storage::new(&config)
        .await
        .expect("Failed to create S3 storage");
    let result = storage
        .put_blob("test-error", Bytes::from(vec![1, 2, 3]))
        .await;
    assert!(result.is_err(), "Expected error with invalid credentials");
}

fn setup_test_config() -> S3Config {
    // Replace these values with your test environment values
    S3Config {
        endpoint: std::env::var("S3_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:9001".to_string()),
        region: std::env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
        bucket: std::env::var("S3_BUCKET").unwrap_or_else(|_| "test-bucket".to_string()),
        auth_method: S3AuthMethod::Static {
            access_key_id: std::env::var("S3_ACCESS_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            secret_access_key: std::env::var("S3_SECRET_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
        },
        use_path_style: true,
        multipart_threshold: Some(5 * 1024 * 1024), // 5MB for testing
        part_size: Some(5 * 1024 * 1024),           // 5MB for testing
        retry_attempts: Some(2),
    }
}
