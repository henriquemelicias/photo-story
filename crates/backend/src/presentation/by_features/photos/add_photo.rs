use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPhotoInput {
    pub url:   String,
    pub title: String,
}

pub async fn add_photo( _input: Json<AddPhotoInput> ) -> impl IntoResponse { unimplemented!() }
