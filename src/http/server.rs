use crate::{
    http::{error::Error, router::RequestIdentifier},
    not_found,
};

use super::{request::HttpStream, router::Router};
use std::{
    cell::RefCell,
    io::Write,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread::JoinHandle,
};

/// Server struct
pub struct Server {
    listener: TcpListener,
    thread_pool: RefCell<Vec<JoinHandle<anyhow::Result<(), super::error::Error>>>>,
    router: Arc<Router>,
}

impl Server {
    #[must_use]
    /// Create a new [```HttpServer```] instance
    pub fn new(listener: TcpListener, router: Router) -> Self {
        Self {
            listener,
            thread_pool: RefCell::new(Vec::new()),
            router: Arc::new(router),
        }
    }

    fn handle_request(
        stream: &mut TcpStream,
        router: &Arc<Router>,
    ) -> anyhow::Result<(), super::error::Error> {
        let response: Vec<u8> = match stream.parse() {
            Ok(request) => {
                let request_identifier = RequestIdentifier {
                    method: request.method,
                    path: request.path.clone(),
                };

                let handler = router.select(&request_identifier);

                let response = handler.map_or(
                    not_found!(
                        method: request_identifier.method,
                        path: request_identifier.path.clone()
                    )
                    .into(),
                    |handler| match handler.handle(request) {
                        Ok(response) => response,
                        Err(error) => error.into(),
                    },
                );

                response.into()
            }
            Err(error) => error.into(),
        };

        stream.write_all(&response)?;

        stream.shutdown(std::net::Shutdown::Both)?;

        Ok(())
    }

    /// Starts listening on the given port
    pub fn listen(&self) {
        eprintln!("\x1B[2J\x1B[1;1H"); // Clear Screen
        eprintln!("Server running on http://localhost:8000");

        loop {
            if let Ok((mut stream, socket_addr)) = self.listener.accept() {
                let router = self.router.clone();

                let thread = std::thread::spawn(move || {
                    eprintln!("Connected to client on {socket_addr:?}");
                    Server::handle_request(&mut stream, &router)
                });

                self.thread_pool.borrow_mut().push(thread);
            }
        }
    }
}
