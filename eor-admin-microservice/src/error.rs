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
            Error::UnavailableEmail => (StatusCode::UNPROCESSABLE_ENTITY, "Email is unavailable"),
            Error::UnsupportedImageFormat => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Image must be PNG or JPEG",
            ),
            Error::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Error::Unauthorized(message) => {
                eprintln!("{message}");
                return StatusCode::UNAUTHORIZED.into_response();
            }
            Error::PayloadTooLarge(message) => (StatusCode::PAYLOAD_TOO_LARGE, message),
            Error::Internal(message) => {
                eprintln!("{message}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
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
