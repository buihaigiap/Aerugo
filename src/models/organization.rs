use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
