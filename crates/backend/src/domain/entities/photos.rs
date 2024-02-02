use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    pub id:          i32,
    pub created_at:  DateTime<Utc>,
    pub uploaded_at: DateTime<Utc>,

    pub url:         String,
    pub title:       String,
    pub description: Option<String>,
}
