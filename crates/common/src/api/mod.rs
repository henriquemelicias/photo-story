use serde::{Deserialize, Serialize};

pub mod photos;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponseBody {
    pub message: String,
}
