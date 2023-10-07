/// Type Definitions for Errors
pub mod error;

/// Type Definitions / Implementations for All things related to Headers
pub mod header;

/// Type Definitions / Implementations for All things related to Http Status codes and HTTP Status Texts
pub mod status;

/// Type Definitions / Implementations for All things related to Http Responses
/// Includes implementations for building a response as well as serializing the response as bytes / string
pub mod response;

/// Type Definitions / Implementations for All things related to Http Requests
/// Includes implementations for parsing responses
pub mod request;

/// Method definitions for HTTP Request
pub mod method;

/// Type Definitions / Implementations for routing
pub mod router;

/// HTTP server Abstraction layer
pub mod server;

pub use dynamo::route;
