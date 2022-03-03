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

macro_rules! from_error {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($error_name:ty) => {
        impl From<$error_name> for Error {
            fn from(e: $error_name) -> Self {
                Error::Internal(e.to_string())
            }
        }
    };
}

from_error!(reqwest::Error);
from_error!(time::error::Format);
from_error!(time::error::ComponentRange);
from_error!(time::error::InvalidFormatDescription);
from_error!(sqlx::Error);
from_error!(strum::ParseError);
