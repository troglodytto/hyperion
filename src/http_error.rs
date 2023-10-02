use std::{num::ParseIntError, string::FromUtf8Error};
use thiserror::Error;

use crate::{
    http_header::HttpHeader,
    http_response::{HttpResponse, Serializeable},
    http_status::HttpStatus,
};

/// Representation of Different types of errors that can happend during the processing of a request
#[derive(Debug, Error)]
pub enum HttpError {
    /// I/O Related Errors
    #[error("Io error")]
    IoError(#[from] std::io::Error),

    /// The User sent an invalid request
    #[error("Bad Request")]
    BadRequest(String),

    /// Something went wrong on the server side
    #[error("Internal Server Error: Failed to parse request")]
    RequestParseError(#[from] FromUtf8Error),
}

impl From<serde_json::Error> for HttpError {
    fn from(value: serde_json::Error) -> Self {
        HttpError::BadRequest(value.to_string())
    }
}

impl TryFrom<HttpError> for HttpResponse<String, Serializeable> {
    type Error = HttpError;

    fn try_from(error: HttpError) -> Result<Self, Self::Error> {
        let body = match &error {
            HttpError::IoError(error) => error.to_string(),
            HttpError::BadRequest(error) => error.clone(),
            HttpError::RequestParseError(error) => error.to_string(),
        };

        let headers = vec![
            HttpHeader::ContentType("text/plain".to_string()),
            HttpHeader::Authorization("Bearer token".to_string()),
        ];

        let status = match &error {
            HttpError::RequestParseError(_) | HttpError::IoError(_) => {
                HttpStatus::InternalServerError
            }
            HttpError::BadRequest(_) => HttpStatus::BadRequest,
        };

        HttpResponse::new(body, headers, status)
    }
}

impl From<ParseIntError> for HttpError {
    fn from(value: ParseIntError) -> Self {
        HttpError::BadRequest(format!("Invalid Integer Value: {value}"))
    }
}

/// Build a HTTP Error from given Error Kind and Description.
#[macro_export]
macro_rules! http_error {
    ($variant:ident, $inner:literal) => {
        $crate::http_error::HttpError::$variant($inner.to_string())
    };
    ($variant:ident, $inner:expr) => {
        $crate::http_error::HttpError::$variant($inner)
    };
}

/// Convert a [`std::io::ErrorKind`] / `&str` pair into an IO Error
#[macro_export]
macro_rules! io_error {
    ($kind:ident, $inner:literal) => {
        Error::new(std::io::ErrorKind::$kind, anyhow!($inner))
    };
}
