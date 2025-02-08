use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use serde_json::json;
use thiserror::Error;

pub mod service;
pub use service::*;
pub mod storage;
pub mod routes;
pub use routes::*;
mod state;
pub use state::*;


#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Key not found")]
    NotFound,

    #[error("Storage error: {0}")]
    StorageFailure(#[from] std::io::Error),

    #[error("Invalid UTF-8 data")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

impl ResponseError for StorageError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            StorageError::NotFound => HttpResponse::NotFound().json(json!({"error": "Key not found"})),
            StorageError::StorageFailure(_) => HttpResponse::InternalServerError().json(json!({"error": "Storage failure"})),
            StorageError::Utf8Error(_) => HttpResponse::BadRequest().json(json!({"error": "Invalid UTF-8"})),
        }
    }
}