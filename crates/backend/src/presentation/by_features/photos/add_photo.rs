use axum::Json;
use axum::response::IntoResponse;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPhotoInput
{
    pub url: String,
    pub title: String,
}

pub async fn add_photo( input: Json<AddPhotoInput> ) -> impl IntoResponse { unimplemented!() }
