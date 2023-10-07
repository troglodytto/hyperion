use std::ops::DerefMut;

use super::{
    method::Method,
    request::Request,
    response::{Body, Response},
};

/// Defining behavior of a HTTP request handler
pub trait RequestHandler: Send + Sync {
    /// A function that handles an HTTP request for a certain method / path pair
    /// # Errors
    /// - On failed handling of response
    fn handle(&self, request: Request) -> Result<Response<Box<dyn Body>>, super::error::Error>;
}

/// Identifies a unique HTTP request based on Method and Path
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestIdentifier {
    /// Method of the request
    pub method: Method,

    /// Path of the request
    pub path: String,
}

/// Routing table for HTTP requests
pub struct Router {
    pub(crate) inner: std::collections::HashMap<RequestIdentifier, Box<dyn RequestHandler>>,
}

impl Router {
    /// Get the designated route for a given request
    #[must_use]
    pub fn select(&self, identifier: &RequestIdentifier) -> Option<&dyn RequestHandler> {
        self.inner.get(identifier).map(std::ops::Deref::deref)
    }
}

impl<const N: usize> From<[(RequestIdentifier, Box<dyn RequestHandler>); N]> for Router {
    fn from(value: [(RequestIdentifier, Box<dyn RequestHandler>); N]) -> Self {
        Self {
            inner: std::collections::HashMap::from(value),
        }
    }
}

/// Creates a new route mapping of `RequestIdentifier` with the given handlers
#[macro_export]
macro_rules! routes {
    ($($method:ident $path:literal -> $handler:ident,)*) => {
        $crate::http::router::Router::from([
            $(($crate::http::router::RequestIdentifier {
                method: $crate::http::method::Method::$method,
                path: $path.to_string(),
            }, Box::new($handler) as Box<dyn hyperion::http::router::RequestHandler>),)*
        ])
    };
}

pub use routes;
