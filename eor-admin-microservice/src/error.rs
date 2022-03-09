//! Error types.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Error responses.
#[derive(Debug)]
pub enum Error {
    UnavailableEmail,
    UnsupportedImageFormat,
    BadRequest(&'static str),
    Unauthorized(&'static str),
    Internal(String),
    PayloadTooLarge(&'static str),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::UnavailableEmail => (StatusCode::CONFLICT, "Email is unavailable"),
            Error::BadRequest(message) => (StatusCode::BAD_REQUEST, "Bad Request"),
            Error::Unauthorized(message) => {
                return StatusCode::UNAUTHORIZED.into_response();
            }
            Error::Internal(message) => {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            Error::UnsupportedImageFormat => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Image must be PNG or JPEG",
            ),
            Error::PayloadTooLarge(message) => (StatusCode::PAYLOAD_TOO_LARGE, message),
        };
        (status, message).into_response()
    }
}

impl<T> From<T> for Error
where
    T: std::error::Error,
{
    fn from(e: T) -> Self {
        Error::Internal(e.to_string())
    }
}
