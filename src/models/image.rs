use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub digest: String,
    pub size: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
