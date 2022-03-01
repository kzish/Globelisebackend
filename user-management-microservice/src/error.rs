//! Error types.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Error responses.
#[derive(Debug)]
pub enum Error {
    Dapr(String),
    Database(String),
    UnavailableEmail,
    WrongUserType,
    UnsupportedImageFormat,
    BadRequest(&'static str),
    Unauthorized(&'static str),
    Forbidden,
    #[allow(dead_code)]
    NotFound,
    PayloadTooLarge(&'static str),
    Internal(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::Dapr(message) => {
                eprintln!("{message}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            Error::Database(message) => {
                eprintln!("{message}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            Error::UnavailableEmail => (StatusCode::UNPROCESSABLE_ENTITY, "Email is unavailable"),
            Error::WrongUserType => return StatusCode::UNAUTHORIZED.into_response(),
            Error::UnsupportedImageFormat => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Image must be PNG or JPEG",
            ),
            Error::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Error::Unauthorized(message) => {
                println!("{message}");
                return StatusCode::UNAUTHORIZED.into_response();
            }
            Error::Forbidden => return StatusCode::FORBIDDEN.into_response(),
            Error::NotFound => return StatusCode::NOT_FOUND.into_response(),
            Error::PayloadTooLarge(message) => (StatusCode::PAYLOAD_TOO_LARGE, message),
            Error::Internal(message) => {
                eprintln!("{message}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        (status, message).into_response()
    }
}

impl From<sqlx::error::Error> for Error {
    fn from(e: sqlx::error::Error) -> Self {
        Error::Database(e.to_string())
    }
}

impl From<time::error::ComponentRange> for Error {
    fn from(e: time::error::ComponentRange) -> Self {
        Error::Internal(e.to_string())
    }
}
