//! Error types.

use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;

pub type GlobeliseResult<T> = std::result::Result<T, GlobeliseError>;

/// Error responses.
#[derive(Debug)]
pub enum GlobeliseError {
    Dapr(String),
    Database(String),
    UnavailableEmail,
    WrongUserType,
    UnsupportedImageFormat,
    BadRequest(&'static str),
    BadRequestOwned(String),
    Unauthorized(&'static str),
    Forbidden,
    #[allow(dead_code)]
    NotFound,
    PayloadTooLarge(&'static str),
    Internal(String),
}

impl GlobeliseError {
    pub fn internal<E>(e: E) -> GlobeliseError
    where
        E: std::error::Error,
    {
        GlobeliseError::Internal(format!("{:#?}", e))
    }
}

impl IntoResponse for GlobeliseError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            GlobeliseError::Dapr(message) => {
                eprintln!("{:#?}", message);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            GlobeliseError::Database(message) => {
                eprintln!("{:#?}", message);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            GlobeliseError::UnavailableEmail => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Email is unavailable")
            }
            GlobeliseError::WrongUserType => return StatusCode::UNAUTHORIZED.into_response(),
            GlobeliseError::UnsupportedImageFormat => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Image must be PNG or JPEG",
            ),
            GlobeliseError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            GlobeliseError::BadRequestOwned(message) => {
                eprintln!("{:#?}", message);
                return StatusCode::BAD_REQUEST.into_response();
            }
            GlobeliseError::Unauthorized(message) => {
                eprintln!("{:#?}", message);
                return StatusCode::UNAUTHORIZED.into_response();
            }
            GlobeliseError::Forbidden => return StatusCode::FORBIDDEN.into_response(),
            GlobeliseError::NotFound => return StatusCode::NOT_FOUND.into_response(),
            GlobeliseError::PayloadTooLarge(message) => (StatusCode::PAYLOAD_TOO_LARGE, message),
            GlobeliseError::Internal(message) => {
                eprintln!("{:#?}", message);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        (status, message).into_response()
    }
}

impl std::fmt::Display for GlobeliseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobeliseError::UnsupportedImageFormat => write!(f, "Image must be PNG or JPEG"),
            GlobeliseError::BadRequest(message) => write!(f, "{message}"),
            GlobeliseError::PayloadTooLarge(message) => write!(f, "{message}"),
            GlobeliseError::Internal(_) => write!(f, "Internal error"),
            _ => writeln!(f, "{:#?}", self),
        }
    }
}

impl<T> From<T> for GlobeliseError
where
    T: std::error::Error,
{
    fn from(e: T) -> Self {
        GlobeliseError::internal(e)
    }
}
