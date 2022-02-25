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
    BadRequest(&'static str),
    Unauthorized(&'static str),
    #[allow(dead_code)]
    NotFound,
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
            Error::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Error::Unauthorized(message) => {
                println!("{message}");
                return StatusCode::UNAUTHORIZED.into_response();
            }
            Error::NotFound => return StatusCode::NOT_FOUND.into_response(),
            Error::Internal(message) => {
                eprintln!("{message}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        (status, message).into_response()
    }
}

impl From<dapr::error::Error> for Error {
    fn from(e: dapr::error::Error) -> Self {
        Error::Dapr(e.to_string())
    }
}
