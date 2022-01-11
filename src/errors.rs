use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    /// The request was unacceptable, often due to an incorrect query string parameter
    #[error("Bad Request: {}", .0.error.message)]
    BadRequest(ErrorEnvelope),
    /// The parameters were valid but the request failed.
    #[error("Request Failed: {}", .0.error.message)]
    RequestFailed(ErrorEnvelope),
    /// The user doesn't have permissions to perform the request.
    #[error("Request Forbidden: {}", .0.error.message)]
    Forbidden(ErrorEnvelope),
    /// The requested resource doesn't exist.
    #[error("Not Found: {}", .0.error.message)]
    NotFound(ErrorEnvelope),
    /// The rate limit has been exceeded.
    #[error("Too Many Requests: {}", .0.error.message)]
    TooManyRequests(ErrorEnvelope),
    /// Something went wrong on our end.
    #[error("Server Error: {}", .0.error.message)]
    ServerError(ErrorEnvelope),
    /// Error parsing the response body
    #[error("Failed to decode the response body")]
    DecodeFailed(#[source] reqwest::Error),
    /// Error occurred before the body could be decoded
    #[error("An error occurred while attempting to make a request.")]
    RequestError(#[source] reqwest::Error),
}

/// The root response body for an error
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorEnvelope {
    pub error: ApiError,
}

/// The details of an error response
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub code: usize,
}

impl From<ErrorEnvelope> for ClientError {
    fn from(e: ErrorEnvelope) -> Self {
        match e.error.code {
            400 => ClientError::BadRequest(e),
            402 => ClientError::RequestFailed(e),
            403 => ClientError::Forbidden(e),
            404 => ClientError::NotFound(e),
            429 => ClientError::TooManyRequests(e),
            500..=504 => ClientError::ServerError(e),
            _ => ClientError::BadRequest(e),
        }
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(e: reqwest::Error) -> Self {
        ClientError::RequestError(e)
    }
}
