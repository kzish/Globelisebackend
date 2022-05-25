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
    BadRequest(String),
    Unauthorized(String),
    Forbidden,
    NotFoundString(String),
    NotFound,
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

    pub fn dapr<S>(s: S) -> GlobeliseError
    where
        S: ToString,
    {
        GlobeliseError::Dapr(s.to_string())
    }

    pub fn not_found<S>(s: S) -> GlobeliseError
    where
        S: ToString,
    {
        GlobeliseError::NotFoundString(s.to_string())
    }
}

impl IntoResponse for GlobeliseError {
    fn into_response(self) -> Response {
        match self {
            GlobeliseError::Dapr(message) => {
                #[cfg(debug_assertions)]
                return (StatusCode::INTERNAL_SERVER_ERROR, message).into_response();
                #[cfg(not(debug_assertions))]
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            GlobeliseError::Database(message) => {
                #[cfg(debug_assertions)]
                return (StatusCode::INTERNAL_SERVER_ERROR, message).into_response();
                #[cfg(not(debug_assertions))]
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            GlobeliseError::UnavailableEmail => {
                #[cfg(debug_assertions)]
                return (StatusCode::UNPROCESSABLE_ENTITY, "Email is unavailable").into_response();
                #[cfg(not(debug_assertions))]
                return StatusCode::UNPROCESSABLE_ENTITY.into_response();
            }
            GlobeliseError::WrongUserType => {
                #[cfg(debug_assertions)]
                return (StatusCode::UNAUTHORIZED, "Wrong user type").into_response();
                #[cfg(not(debug_assertions))]
                return StatusCode::UNAUTHORIZED.into_response();
            }
            GlobeliseError::UnsupportedImageFormat => {
                #[cfg(debug_assertions)]
                return (
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    "Image must be PNG or JPEG",
                )
                    .into_response();
                #[cfg(not(debug_assertions))]
                return StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response();
            }
            GlobeliseError::BadRequest(message) => {
                #[cfg(debug_assertions)]
                return (StatusCode::BAD_REQUEST, message).into_response();
                #[cfg(not(debug_assertions))]
                return StatusCode::BAD_REQUEST.into_response();
            }
            GlobeliseError::Unauthorized(message) => {
                #[cfg(debug_assertions)]
                return (StatusCode::UNAUTHORIZED, message).into_response();
                #[cfg(not(debug_assertions))]
                return StatusCode::UNAUTHORIZED.into_response();
            }
            GlobeliseError::Forbidden => {
                #[cfg(debug_assertions)]
                return StatusCode::FORBIDDEN.into_response();
                #[cfg(not(debug_assertions))]
                return StatusCode::FORBIDDEN.into_response();
            }
            GlobeliseError::NotFound => {
                #[cfg(debug_assertions)]
                return StatusCode::NOT_FOUND.into_response();
                #[cfg(not(debug_assertions))]
                return StatusCode::NOT_FOUND.into_response();
            }
            GlobeliseError::NotFoundString(message) => {
                #[cfg(debug_assertions)]
                return (StatusCode::NOT_FOUND, message).into_response();
                #[cfg(not(debug_assertions))]
                return StatusCode::NOT_FOUND.into_response();
            }
            GlobeliseError::PayloadTooLarge(message) => {
                #[cfg(debug_assertions)]
                return (StatusCode::PAYLOAD_TOO_LARGE, message).into_response();
                #[cfg(not(debug_assertions))]
                return StatusCode::PAYLOAD_TOO_LARGE.into_response();
            }
            GlobeliseError::Internal(message) => {
                #[cfg(debug_assertions)]
                return (StatusCode::INTERNAL_SERVER_ERROR, message).into_response();
                #[cfg(not(debug_assertions))]
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
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
