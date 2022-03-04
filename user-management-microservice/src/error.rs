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
                eprintln!("{message}");
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

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

macro_rules! impl_from_error {
    ($error_type:ty) => {
        impl From<$error_type> for crate::error::Error {
            fn from(e: $error_type) -> Self {
                Error::Internal(e.to_string())
            }
        }
    };
}

impl_from_error!(sqlx::Error);
impl_from_error!(dapr::error::Error);
impl_from_error!(email_address::Error);
impl_from_error!(reqwest::Error);
impl_from_error!(time::error::Format);
impl_from_error!(time::error::ComponentRange);
impl_from_error!(time::error::InvalidFormatDescription);
impl_from_error!(strum::ParseError);
impl_from_error!(jsonwebtoken::errors::Error);
impl_from_error!(eor_admin_microservice_sdk::Error);
