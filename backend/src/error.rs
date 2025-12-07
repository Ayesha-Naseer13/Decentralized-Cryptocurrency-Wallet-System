use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    InternalError(String),
    InvalidSignature,
    InvalidWalletId,
    InsufficientBalance,
    DatabaseError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            ApiError::InvalidSignature => write!(f, "Invalid signature"),
            ApiError::InvalidWalletId => write!(f, "Invalid wallet ID"),
            ApiError::InsufficientBalance => write!(f, "Insufficient balance"),
            ApiError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            ApiError::InvalidSignature => (StatusCode::BAD_REQUEST, "Invalid signature".to_string()),
            ApiError::InvalidWalletId => (StatusCode::BAD_REQUEST, "Invalid wallet ID".to_string()),
            ApiError::InsufficientBalance => (StatusCode::BAD_REQUEST, "Insufficient balance".to_string()),
            ApiError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        HttpResponse::build(status).json(json!({
            "error": message
        }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InvalidSignature => StatusCode::BAD_REQUEST,
            ApiError::InvalidWalletId => StatusCode::BAD_REQUEST,
            ApiError::InsufficientBalance => StatusCode::BAD_REQUEST,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
