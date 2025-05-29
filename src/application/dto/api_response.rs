use serde::Serialize;
use uuid::Uuid;

// ApiResponse is a generic struct used to standardize API responses. It contains:
// - transaction_id: A unique identifier for each API request/response.
// - code: The status code of the response (e.g., 200 for success, 400 for error).
// - message: A human-readable message providing more context about the response.
// - data: The actual response data, which is of generic type T (can be any type).
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub transaction_id: String,
    pub code: i16,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    // success() creates a new ApiResponse for a successful request.
    // It automatically sets the code to 200 (OK) and allows custom data and message.
    pub fn success(data: T, message: &str) -> Self {
        Self {
            transaction_id: Uuid::new_v4().to_string(), // Generate a new unique transaction ID.
            code: 200,                                  // HTTP Status code 200 (OK).
            message: message.to_string(),               // Custom success message.
            data,                                       // The provided data for the response.
        }
    }

    // error() creates a new ApiResponse for an error.
    // It allows you to specify the status code (e.g., 400, 404) and an error message.
    pub fn error(code: i16, message: &str, data: T) -> Self {
        Self {
            transaction_id: Uuid::new_v4().to_string(), // Generate a new unique transaction ID.
            code,                         // Custom error code (e.g., 400 for Bad Request).
            message: message.to_string(), // Custom error message.
            data, // The data associated with the error response (could be empty or an error-specific object).
        }
    }
}
