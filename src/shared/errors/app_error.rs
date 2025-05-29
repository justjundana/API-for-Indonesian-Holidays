use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::fmt;

// Type alias for Result with AppError as the error type
pub type AppResult<T> = Result<T, AppError>;

// Enum defining different error types in the application
#[derive(Debug)]
pub enum AppError {
    NotFound(String),        // 404 error: Resource not found
    InternalServer(String),  // 500 error: Internal server issues
    BadRequest(String),      // 400 error: Bad request from client
    ExternalService(String), // 502 error: External service failure
    Serialization(String),   // Error during serialization (e.g., JSON parsing)
}

// Implementing Display trait for AppError to format error messages as strings
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::InternalServer(msg) => write!(f, "Internal Server Error: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::ExternalService(msg) => write!(f, "External Service Error: {}", msg),
            AppError::Serialization(msg) => write!(f, "Serialization Error: {}", msg),
        }
    }
}

// Implementing IntoResponse trait to convert AppError into an HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Map error to corresponding HTTP status and message
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServer(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ExternalService(msg) => (StatusCode::BAD_GATEWAY, msg),
            AppError::Serialization(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        // Create a JSON body for the error response
        let body = Json(json!({
            "error": true,
            "message": message,
            "code": status.as_u16()
        }));

        // Return the response with status and JSON body
        (status, body).into_response()
    }
}

// Converting std::io::Error to AppError
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        // Wrap IO error as InternalServer error
        AppError::InternalServer(format!("IO Error: {}", err))
    }
}

// Converting serde_json::Error to AppError
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        // Wrap JSON parsing error as Serialization error
        AppError::Serialization(format!("JSON Error: {}", err))
    }
}

// Converting reqwest::Error to AppError
impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        // Wrap HTTP request error as ExternalService error
        AppError::ExternalService(format!("HTTP Request Error: {}", err))
    }
}
