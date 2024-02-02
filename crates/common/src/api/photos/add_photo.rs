use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize)]
pub struct Input {
    pub url:         String,
    pub title:       String,
    pub description: Option<String>,
}
