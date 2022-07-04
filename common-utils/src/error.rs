//! Error types.

use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;

pub type GlobeliseResult<T> = std::result::Result<T, GlobeliseError>;

/// Error responses.
#[derive(Debug)]
pub enum GlobeliseError {
    UnavailableEmail,
    WrongUserType,
    UnsupportedImageFormat,
    BadRequest(String),
    Unauthorized(String),
    Forbidden,
    NotFound(String),
    PayloadTooLarge(String),
    Internal(String),
}

impl GlobeliseError {
    pub fn internal<S>(s: S) -> GlobeliseError
    where
        S: ToString,
    {
        GlobeliseError::Internal(s.to_string())
    }

    pub fn bad_request<S>(s: S) -> GlobeliseError
    where
        S: ToString,
    {
        GlobeliseError::BadRequest(s.to_string())
    }

    pub fn unauthorized<S>(s: S) -> GlobeliseError
    where
        S: ToString,
    {
        GlobeliseError::Unauthorized(s.to_string())
    }

    pub fn payload_too_large<S>(s: S) -> GlobeliseError
    where
        S: ToString,
    {
        GlobeliseError::PayloadTooLarge(s.to_string())
    }

    pub fn not_found<S>(s: S) -> GlobeliseError
    where
        S: ToString,
    {
        GlobeliseError::NotFound(s.to_string())
    }
}

impl IntoResponse for GlobeliseError {
    fn into_response(self) -> Response {
        match self {
            GlobeliseError::UnavailableEmail => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Email is unavailable").into_response()
            }
            GlobeliseError::WrongUserType => {
                (StatusCode::UNAUTHORIZED, "Wrong user type").into_response()
            }
            GlobeliseError::UnsupportedImageFormat => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "Image must be PNG or JPEG",
            )
                .into_response(),
            GlobeliseError::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, message).into_response()
            }
            GlobeliseError::Unauthorized(message) => {
                (StatusCode::UNAUTHORIZED, message).into_response()
            }
            GlobeliseError::Forbidden => StatusCode::FORBIDDEN.into_response(),
            GlobeliseError::NotFound(message) => (StatusCode::NOT_FOUND, message).into_response(),
            GlobeliseError::PayloadTooLarge(message) => {
                (StatusCode::PAYLOAD_TOO_LARGE, message).into_response()
            }
            GlobeliseError::Internal(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
        }
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
        GlobeliseError::internal(format!("{:#?}", e))
    }
}
