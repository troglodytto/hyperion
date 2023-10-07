use crate::http::{header::HttpHeader, status::HttpStatus};
use serde::Serialize;
use std::{
    borrow::BorrowMut,
    fmt::{Debug, Display},
};

/// Representation of an HTTP body
pub trait Body: Debug {
    /// Convert the body into bytes
    fn bytes(&self) -> Vec<u8>;

    /// Return the size of the body in bytes
    fn content_length(&self) -> usize {
        self.bytes().len()
    }
}

impl<T: Body> Body for Option<T> {
    fn bytes(&self) -> Vec<u8> {
        if let Some(s) = self {
            s.bytes()
        } else {
            vec![]
        }
    }
}

impl Body for String {
    fn bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Body for &str {
    fn bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Body for () {
    fn bytes(&self) -> Vec<u8> {
        vec![]
    }

    fn content_length(&self) -> usize {
        0
    }
}

impl Body for Vec<u8> {
    fn bytes(&self) -> Vec<u8> {
        self.clone()
    }

    fn content_length(&self) -> usize {
        self.len()
    }
}

impl Body for Box<dyn Body> {
    fn bytes(&self) -> Vec<u8> {
        Body::bytes(self.as_ref())
    }
}

impl<const N: usize> Body for [u8; N] {
    fn bytes(&self) -> Vec<u8> {
        self.to_vec()
    }

    fn content_length(&self) -> usize {
        N
    }
}

/// Builder for creating a response
#[must_use]
pub struct Builder {
    headers: Vec<HttpHeader>,
    status: HttpStatus,
}

impl Builder {
    /// Sets the Content-Type header of the response
    pub fn content_type(&mut self, value: &str) {
        let existing_header = self
            .headers
            .iter_mut()
            .find(|header| matches!(header, HttpHeader::ContentType(_)));
        let new_header = HttpHeader::ContentType(value.to_string());

        if let Some(header) = existing_header {
            let _ = std::mem::replace(header, new_header);
        } else {
            self.headers.push(new_header);
        }
    }

    /// Returns a Response object with a JSON body
    pub fn json<T: Serialize>(mut self, body: T) -> Response<String> {
        self.content_type("application/json");

        match serde_json::to_string(&body) {
            Ok(body) => Response {
                body,
                headers: self.headers,
                status: self.status,
            },
            Err(error) => Response {
                body: error.to_string(),
                headers: vec![],
                status: HttpStatus::InternalServerError,
            },
        }
    }

    /// Return the response in its current state as-it-is
    pub fn finish(self) -> Response<()> {
        Response {
            body: (),
            headers: self.headers,
            status: self.status,
        }
    }

    /// Returns a Response object with a text body
    pub fn text(self, body: &str) -> Response<&str> {
        Response {
            body,
            headers: self.headers,
            status: self.status,
        }
    }
}

macro_rules! static_response {
    ($status:ident) => {
        /// Creates a new [```HttpResponse```] with a predefined status code
        #[allow(non_snake_case)]
        pub fn $status() -> $crate::http::response::Builder {
            $crate::http::response::Builder {
                headers: vec![],
                status: $crate::http::status::HttpStatus::$status,
            }
        }
    };
}

impl Response<()> {
    static_response!(Continue);
    static_response!(SwitchingProtocols);
    static_response!(Processing);
    static_response!(EarlyHints);

    static_response!(Ok);
    static_response!(Created);
    static_response!(Accepted);
    static_response!(NonAuthoritativeInformation);
    static_response!(NoContent);
    static_response!(ResetContent);
    static_response!(PartialContent);
    static_response!(MultiStatus);
    static_response!(AlreadyReported);
    static_response!(IMUsed);

    static_response!(MultipleChoices);
    static_response!(MovedPermanently);
    static_response!(Found);
    static_response!(NotModified);
    static_response!(UseProxy);
    static_response!(Unused);
    static_response!(TemporaryRedirect);
    static_response!(PermanentRedirect);

    static_response!(BadRequest);
    static_response!(Unauthorized);
    static_response!(PaymentRequired);
    static_response!(Forbidden);
    static_response!(NotFound);
    static_response!(MethodNotAllowed);
    static_response!(NotAcceptable);
    static_response!(ProxyAuthenticationRequired);
    static_response!(RequestTimeout);
    static_response!(Conflict);
    static_response!(Gone);
    static_response!(LengthRequired);
    static_response!(PreconditionFailed);
    static_response!(PayloadTooLarge);
    static_response!(URITooLong);
    static_response!(UnsupportedMediaType);
    static_response!(RangeNotSatisfiable);
    static_response!(ExpectationFailed);
    static_response!(Teapot);
    static_response!(MisdirectedRequest);
    static_response!(UnprocessableEntity);
    static_response!(Locked);
    static_response!(FailedDependency);
    static_response!(TooEarly);
    static_response!(UpgradeRequired);
    static_response!(PreconditionRequired);
    static_response!(TooManyRequests);
    static_response!(RequestHeaderFieldsTooLarge);
    static_response!(UnavailableForLegalReasons);

    static_response!(InternalServerError);
    static_response!(NotImplemented);
    static_response!(BadGateway);
    static_response!(ServiceUnavailable);
    static_response!(GatewayTimeout);
    static_response!(HttpVersionNotSupported);
    static_response!(VariantAlsoNegotiates);
    static_response!(InsufficientStorage);
    static_response!(LoopDetected);
    static_response!(NotExtended);
    static_response!(NetworkAuthenticationRequired);
}

/// Representation of an HTTP Response sent by a server
#[must_use]
#[derive(Debug)]
pub struct Response<T: Body = ()> {
    /// The Body of the HTTP Response
    pub body: T,

    /// Headers sent to the HTTP Response
    pub headers: Vec<HttpHeader>,

    /// Status code of the HTTP Response
    pub status: HttpStatus,
}

impl<T: Body> Response<T> {
    /// Create a new HTTP Response
    pub fn new(body: T, headers: Vec<HttpHeader>, status: HttpStatus) -> Self {
        Self {
            body,
            headers,
            status,
        }
    }
}

impl<T: Body> Display for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = u16::from(&self.status);

        let status_text: &str = self.status.as_ref();

        write!(f, "HTTP/1.1 {status} {status_text}\r\n")?;

        for header in &self.headers {
            write!(f, "{header}\r\n")?;
        }

        write!(f, "\r\n")?;

        Ok(())
    }
}

impl<T: Body> From<Response<T>> for Vec<u8> {
    fn from(val: Response<T>) -> Self {
        let mut response = format!("{val}").as_bytes().to_vec();

        response.extend(val.body.bytes());

        response
    }
}
