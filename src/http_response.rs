use crate::http_error::HttpError;
use crate::{http_header::HttpHeader, http_status::HttpStatus};
use serde::Serialize;
use std::fmt::Display;
use std::marker::PhantomData;

/// Marker Type to indicate that the request is serializable
pub struct Serializeable;

/// Representation of an HTTP Response sent by a server
#[derive(Debug)]
pub struct HttpResponse<T, S> {
    /// The Body of the HTTP Response
    pub body: T,

    /// Headers sent to the HTTP Response
    pub headers: Vec<HttpHeader>,

    /// Status code of the HTTP Response
    pub status: HttpStatus,

    pub(crate) _marker: PhantomData<S>,
    serialized_body: String,
}

impl<T: Serialize> HttpResponse<T, Serializeable> {
    /// Create a new HTTP Response
    /// # Errors
    /// - Failed to serialize the body
    pub fn new(body: T, headers: Vec<HttpHeader>, status: HttpStatus) -> Result<Self, HttpError> {
        let mut response = Self {
            _marker: PhantomData::<Serializeable>,
            body,
            headers,
            status,
            serialized_body: String::new(),
        };

        if !response
            .headers
            .iter()
            .any(|header| matches!(&header, HttpHeader::ContentLength(_)))
        {
            response.serialized_body = serde_json::to_string(&response.body)?;

            response.headers.push(HttpHeader::ContentLength(
                response.serialized_body.len().to_string(),
            ));
        }

        Ok(response)
    }
}

impl<T: Serialize> Display for HttpResponse<T, Serializeable> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = u16::from(&self.status);

        let status_text: &str = self.status.as_ref();

        write!(f, "HTTP/1.1 {status} {status_text}\r\n")?;

        for header in &self.headers {
            write!(f, "{header}\r\n")?;
        }

        write!(f, "\r\n")?;

        write!(f, "{}", self.serialized_body)?;

        Ok(())
    }
}

impl<T: Serialize> From<HttpResponse<T, Serializeable>> for Vec<u8> {
    fn from(val: HttpResponse<T, Serializeable>) -> Self {
        format!("{val}").as_bytes().to_vec()
    }
}
