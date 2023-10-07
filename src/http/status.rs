macro_rules! status_impl {
    ($($status_code:literal $status_text:literal -> $variant:ident,)*) => {
        /// Status code definitions for HTTP Responses
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum HttpStatus {
            $(
                /// HTTP `
                #[doc = stringify!($status_code)]
                #[doc = $status_text]
                ///` status
                $variant,
            )*
        }

        impl From<&HttpStatus> for u16 {
            fn from(value: &HttpStatus) -> Self {
                match value {
                    $($crate::http::status::HttpStatus::$variant => $status_code,)*
                }
            }
        }

        impl AsRef<str> for HttpStatus {
            fn as_ref(&self) -> &str {
                match self {
                    $($crate::http::status::HttpStatus::$variant => $status_text,)*
                }
            }
        }
    }
}

status_impl!(
    100 "Continue" -> Continue,
    101 "Switching Protocols" -> SwitchingProtocols,
    102 "Processing" -> Processing,
    103 "Early Hints" -> EarlyHints,

    200 "Ok" -> Ok,
    201 "Created" -> Created,
    202 "Accepted" -> Accepted,
    203 "Non-Authoritative Information" -> NonAuthoritativeInformation,
    204 "No Content" -> NoContent,
    205 "Reset Content" -> ResetContent,
    206 "Partial Content" -> PartialContent,
    207 "Multi-Status" -> MultiStatus,
    208 "Already Reported" -> AlreadyReported,
    226 "IM Used" -> IMUsed,

    300 "Multiple Choices" -> MultipleChoices,
    301 "Moved Permanently" -> MovedPermanently,
    302 "Found" -> Found,
    304 "Not Modified" -> NotModified,
    305 "Use Proxy" -> UseProxy,
    306 "Unused" -> Unused,
    307 "Temporary Redirect" -> TemporaryRedirect,
    308 "Permanent Redirect" -> PermanentRedirect,

    400 "Bad Request" -> BadRequest,
    401 "Unauthorized" -> Unauthorized,
    402 "Payment Required" -> PaymentRequired,
    403 "Forbidden" -> Forbidden,
    404 "Not Found" -> NotFound,
    405 "Method Not Allowed" -> MethodNotAllowed,
    406 "Not Acceptable" -> NotAcceptable,
    407 "Proxy Authentication Required" -> ProxyAuthenticationRequired,
    408 "Request Timeout" -> RequestTimeout,
    409 "Conflict" -> Conflict,
    410 "Gone" -> Gone,
    411 "Length Required" -> LengthRequired,
    412 "Precondition Failed" -> PreconditionFailed,
    413 "Payload Too Large" -> PayloadTooLarge,
    414 "URI Too Long" -> URITooLong,
    415 "Unsupported Media Type" -> UnsupportedMediaType,
    416 "Range not Satisfiable" -> RangeNotSatisfiable,
    417 "Expectation Failed" -> ExpectationFailed,
    418 "I'm a teapot" -> Teapot,
    421 "Misdirected Request" -> MisdirectedRequest,
    422 "Unprocessable Entity" -> UnprocessableEntity,
    423 "Locked" -> Locked,
    424 "Failed Dependency" -> FailedDependency,
    425 "Too Early" -> TooEarly,
    426 "Upgrade Required" -> UpgradeRequired,
    428 "Precondition Required" -> PreconditionRequired,
    429 "Too Many Requests" -> TooManyRequests,
    431 "Request Header Fields Too Large" -> RequestHeaderFieldsTooLarge,
    451 "Unavailable For Legal Reasons" -> UnavailableForLegalReasons,

    500 "Internal Server Error" -> InternalServerError,
    501 "Not Implemented" -> NotImplemented,
    502 "Bad Gateway" -> BadGateway,
    503 "Service Unavailable" -> ServiceUnavailable,
    504 "Gateway Timeout" -> GatewayTimeout,
    505 "HTTP Version not Supported" -> HttpVersionNotSupported,
    506 "Variant Also Negotiates" -> VariantAlsoNegotiates,
    507 "Insufficient Storage" -> InsufficientStorage,
    508 "Loop Detected" -> LoopDetected,
    510 "Not Extended" -> NotExtended,
    511 "Network Authentication Required" -> NetworkAuthenticationRequired,
);
