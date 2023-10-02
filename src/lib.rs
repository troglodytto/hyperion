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

mod tls;

use crate::http_header::HttpHeader;
use crate::http_request::HttpStream;
use crate::http_response::HttpResponse;
use crate::http_router::RequestIdentifier;
use crate::http_status::HttpStatus;
use crate::tls::cipher_suite::{self, CipherSuite};
use crate::tls::compression_methods::CompressionMethods;
use crate::tls::{HandshakeHeader, RecordHeader, TLSVersion};
use http_router::HttpRouter;
use std::io::Read;
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
    #[allow(clippy::too_many_lines)]
    pub fn listen(&self) {
        eprintln!("\x1B[2J\x1B[1;1H"); // Clear Screen
        eprintln!("Server running on http://localhost:8000");

        loop {
            if let Ok((mut stream, _socket_addr)) = self.listener.accept() {
                let router = self.router.clone();

                let thread = std::thread::spawn(move || {
                    let mut buffer = vec![0; 512];
                    let read_byte_count = stream.read(&mut buffer).unwrap();

                    let mut cursor = 0;

                    let record_header = RecordHeader::from(&buffer[cursor..cursor + 5]);
                    cursor += 5;
                    dbg!(record_header);

                    let handshake_header = HandshakeHeader::from(&buffer[cursor..cursor + 4]);
                    cursor += 4;
                    dbg!(&handshake_header);

                    let client_version = TLSVersion::from([buffer[cursor], buffer[cursor + 1]]);
                    cursor += 1;
                    dbg!(&client_version);

                    let client_random = &buffer[cursor..cursor + 32];
                    cursor += 32;

                    println!(
                        "[{}:{}] client_random = {client_random:0>2x?}",
                        file!(),
                        line!()
                    );

                    let session_id_length = buffer[cursor + 1];
                    cursor += 1;
                    dbg!(session_id_length);

                    let session_id = &buffer[cursor..cursor + session_id_length as usize];
                    cursor += session_id_length as usize;

                    println!("[{}:{}] session_id = {session_id:0>2x?}", file!(), line!());

                    let cipher_suite_byte_length =
                        u16::from_ne_bytes([buffer[cursor + 2], buffer[cursor + 1]]);
                    cursor += 2;
                    dbg!(cipher_suite_byte_length);

                    let cipher_suite =
                        &buffer[cursor + 1..cursor + 1 + cipher_suite_byte_length as usize];
                    cursor += cipher_suite_byte_length as usize + 1;

                    let cipher_suite = cipher_suite.chunks(2).map(CipherSuite::from);

                    let cipher_suite_length = cipher_suite.len();

                    dbg!(cipher_suite_length);

                    println!("[{}:{}] cipher_suites:", file!(), line!());

                    for suite in cipher_suite {
                        println!("  - {suite:?}");
                    }

                    let compression_methods_length = buffer[cursor];
                    dbg!(compression_methods_length);
                    cursor += 1;

                    let compression_methods = buffer
                        [cursor..cursor + compression_methods_length as usize]
                        .iter()
                        .map(|method| CompressionMethods::from(*method));

                    cursor += compression_methods_length as usize;

                    println!("[{}:{}] compression_methods:", file!(), line!());

                    for compression_method in compression_methods {
                        println!("  - {compression_method:?}");
                    }

                    let total_extensions_length =
                        u16::from_ne_bytes([buffer[cursor + 1], buffer[cursor + 2]]);
                    cursor += 2;

                    dbg!(total_extensions_length);

                    // @todo Make the following Dynamic
                    let extension_type =
                        u16::from_ne_bytes([buffer[cursor + 1], buffer[cursor + 2]]);
                    cursor += 2;

                    dbg!(extension_type);

                    let extension_data_length =
                        u16::from_ne_bytes([buffer[cursor + 1], buffer[cursor + 2]]);
                    cursor += 2;

                    dbg!(extension_data_length);

                    let extension_list_entry_length =
                        u16::from_ne_bytes([buffer[cursor + 1], buffer[cursor + 2]]);
                    cursor += 2;

                    dbg!(extension_list_entry_length);

                    let extension_list_entry_type = buffer[cursor];
                    cursor += 1;
                    dbg!(extension_list_entry_type);

                    let hostname_length = u16::from_ne_bytes([buffer[cursor + 1], buffer[cursor]]);
                    cursor += 2;

                    dbg!(hostname_length);

                    let hostname = &buffer[cursor..cursor + hostname_length as usize];

                    if let Ok(hostname) = std::str::from_utf8(hostname) {
                        dbg!(hostname);
                    };

                    for (idx, byte) in buffer[5..].iter().enumerate() {
                        if idx > cursor - 5 {
                            print!("{byte:0>2x} ");
                        } else {
                            print!("⬛ ");
                        }

                        if idx % 16 == 15 {
                            println!();
                        }
                    }
                });

                // self.thread_pool.borrow_mut().push(thread);
            }
        }
    }
}
