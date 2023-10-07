use std::{num::ParseIntError, string::FromUtf8Error};
use thiserror::Error;

use crate::http::{header::HttpHeader, response::Response, status::HttpStatus};

use super::{method::Method, response::Body};

/// Representation of Different types of errors that can happened during the processing of a request
#[derive(Debug, Error)]
pub enum Error {
    /// I/O Related Errors
    #[error("Io error")]
    IoError(#[from] std::io::Error),

    /// The User sent an invalid request
    #[error("Bad Request")]
    BadRequest(String),

    /// The User sent an invalid request
    #[error("Not Found")]
    NotFound {
        /// The method of the request
        method: Method,
        /// The path of the request
        path: String,
    },

    /// Something went wrong on the server side
    #[error("Internal Server Error: Failed to parse request")]
    RequestParseError(#[from] FromUtf8Error),
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::BadRequest(value.to_string())
    }
}

impl From<Error> for Response<Box<dyn Body>> {
    fn from(error: Error) -> Self {
        let body = match &error {
            Error::IoError(error) => error.to_string(),
            Error::BadRequest(error) => error.clone(),
            Error::RequestParseError(error) => error.to_string(),
            Error::NotFound { method, path } => {
                format!("The path {method:?} {path} was not found")
            }
        };

        let headers = vec![
            HttpHeader::ContentType("text/plain".to_string()),
            HttpHeader::Authorization("Bearer token".to_string()),
        ];

        let status = match &error {
            Error::RequestParseError(_) | Error::IoError(_) => HttpStatus::InternalServerError,
            Error::BadRequest(_) => HttpStatus::BadRequest,
            Error::NotFound { .. } => HttpStatus::NotFound,
        };

        Response::new(Box::new(body), headers, status)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::BadRequest(format!("Invalid Integer Value: {value}"))
    }
}

impl From<Error> for Vec<u8> {
    fn from(value: Error) -> Self {
        let response: Response<_> = value.into();

        response.into()
    }
}

/// Build a HTTP Error from given Error Kind and Description.
#[macro_export]
macro_rules! error {
    ($variant:ident, $inner:literal) => {
        $crate::http::error::Error::$variant($inner.to_string())
    };
    ($variant:ident, $inner:expr) => {
        $crate::http::error::Error::$variant($inner)
    };
}

/// Build a HTTP 404 Not Found Error from the given path segment and HTTP Method
#[macro_export]
macro_rules! not_found {
    (method: $method:expr, path: $path:expr) => {
        Error::NotFound {
            method: $method,
            path: $path.clone(),
        }
    };
}

/// Convert a [`std::io::ErrorKind`] / `&str` pair into an IO Error
#[allow(clippy::module_name_repetitions)]
#[macro_export]
macro_rules! io_error {
    ($kind:ident, $inner:literal) => {
        Error::new(std::io::ErrorKind::$kind, anyhow!($inner))
    };
}
