#![deny(clippy::pedantic)]
#![deny(missing_docs)]
#![deny(clippy::missing_panics_doc)]
//! Hyperion: HTTP Server in Rust

/// Type Definitions for Errors
pub mod http_error;

/// Type Definitions / Impls for All things related to Headers
pub mod http_header;

/// Type Definitions / Impls for All things related to Http Status codes and HTTP Status Texts
pub mod http_status;

/// Type Definitions / Impls for All things related to Http Responses
/// Includes implementations for building a response as well as serialising the response as bytes / string
pub mod http_response;

/// Type Definitions / Impls for All things related to Http Requests
/// Includes implementations for parsing responses
pub mod http_request;

/// Method definitions for HTTP Request
pub mod http_method;

/// Type Definitions / Impls for rounting
pub mod http_router;

use http_router::HttpRouter;

use crate::http_header::HttpHeader;
use crate::http_request::HttpStream;
use crate::http_response::HttpResponse;
use crate::http_router::{RequestHandler, RequestIdentifier};
use crate::http_status::HttpStatus;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::{cell::RefCell, io::Write};

fn handle_request(
    stream: &mut TcpStream,
    router: &Arc<HttpRouter>,
) -> anyhow::Result<(), http_error::HttpError> {
    let request = stream.parse();

    let response_bytes: Vec<u8> = match request {
        Ok(request) => {
            let identifier = RequestIdentifier {
                method: request.method,
                path: request.path.clone(),
            };

            if let Some(handler) = router.get(&identifier) {
                let response = handler(&request)?;
                response.into()
            } else {
                HttpResponse::new("NOT FOUND MATE".to_string(), vec![], HttpStatus::NotFound)?
                    .into()
            }
        }
        Err(error) => {
            let response: HttpResponse<String, _> = error.try_into()?;

            response.into()
        }
    };

    stream.write_all(&response_bytes)?;

    stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}

/// Server struct
pub struct HttpServer {
    listener: TcpListener,
    thread_pool: RefCell<Vec<JoinHandle<anyhow::Result<(), http_error::HttpError>>>>,
    router: Arc<HttpRouter>,
}

impl HttpServer {
    #[must_use]
    /// Create a new [```HttpServer```] instance
    pub fn new(listener: TcpListener, router: HttpRouter) -> Self {
        Self {
            listener,
            thread_pool: RefCell::new(Vec::new()),
            router: Arc::new(router),
        }
    }

    /// Starts listening on the given port
    pub fn listen(&self) {
        eprintln!("\x1B[2J\x1B[1;1H"); // Clear Screen
        eprintln!("Server running on http://localhost:8000");

        loop {
            if let Ok((mut stream, _socket_addr)) = self.listener.accept() {
                let router = self.router.clone();

                let thread = std::thread::spawn(move || handle_request(&mut stream, &router));

                self.thread_pool.borrow_mut().push(thread);
            }
        }
    }
}
