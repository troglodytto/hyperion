use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

macro_rules! header_impl {
    ($($rfc_section:literal $name:literal -> $variant:ident,)*) => {
        /// Representation of a HTTP Header
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
        #[serde(rename_all = "kebab-case")]
        pub enum HttpHeader {
            $(
                /// HTTP `
                #[doc = $name]
                ///` header As defined in RFC 2616
                #[doc = stringify!(Section $name)]
                $variant(String),
            )*
        }

        impl HttpHeader {
            /// Creates a new HTTP header with the given name and value
            /// # Errors
            /// - Invalid header name provided
            // @todo - Add Validation for Header Values
            pub fn new(name: &str, value: &str) -> Result<HttpHeader> {
                match name {
                    $($name => Ok(HttpHeader::$variant(value.to_string())),)*
                    header => Err(anyhow!("Unknown header '{header}'")),
                }
            }

            /// Returns the RFC name for the header
            #[must_use]
            pub fn name(&self) -> String {
                match self {
                    $($crate::http::header::HttpHeader::$variant(_) => $name.to_string(),)*
                }
            }

            /// Returns the Value of the header
            #[must_use]
            pub fn value(&self) -> String {
                match self {
                    $($crate::http::header::HttpHeader::$variant(value) => value.to_string(),)*
                }
            }
        }
    };
}

header_impl!(
    14.1 "Accept" -> Accept,
    14.2 "Accept-Charset" -> AcceptCharset,
    14.3 "Accept-Encoding" -> AcceptEncoding,
    14.4 "Accept-Language" -> AcceptLanguage,
    14.5 "Accept-Ranges" -> AcceptRanges,
    14.6 "Age" -> Age,
    14.9 "Cache-Control" -> CacheControl,
    14.8 "Authorization" -> Authorization,
    14.10 "Connection" -> Connection,
    14.13 "Content-Length" -> ContentLength,
    14.16 "Content-Range" -> ContentRange,
    14.17 "Content-Type" -> ContentType,
    14.18 "Date" -> Date,
    14.23 "Host" -> Host,
    14.32 "Pragma" -> Pragma,
    14.43 "User-Agent" -> UserAgent,
);

impl Display for HttpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header_name = self.name();
        let value = self.value();

        write!(f, "{header_name}: {value}")
    }
}
