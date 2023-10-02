use crate::http_error::HttpError;
use crate::http_method::HttpMethod;
use crate::http_request::HttpRequest;
use crate::http_response::{HttpResponse, Serializeable};

/// A function that handles an HTTP request for a certain method / path pair
pub type RequestHandler =
    fn(&HttpRequest) -> Result<HttpResponse<String, Serializeable>, HttpError>;

/// Identifies a unique HTTP request based on Method and Path
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestIdentifier {
    /// Method of the request
    pub method: HttpMethod,

    /// Path of the request
    pub path: String,
}

/// Routing table for HTTP requests
pub type HttpRouter = std::collections::HashMap<RequestIdentifier, RequestHandler>;

/// Creates a new route mapping of `RequestIdentifier` with the given handlers
#[macro_export]
macro_rules! routes {
    ($($method:ident, $path:literal -> $handler:ident,)*) => {
        $crate::http_router::HttpRouter::from([
            $(($crate::http_router::RequestIdentifier {
                method: $crate::http_method::HttpMethod::$method,
                path: $path.to_string(),
            }, $handler as $crate::http_router::RequestHandler),)*
        ])
    };
}
