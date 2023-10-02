#![deny(clippy::pedantic)]
#![deny(clippy::missing_panics_doc)]

use hyperion::{
    http_error::HttpError,
    http_header::HttpHeader,
    http_request::HttpRequest,
    http_response::{HttpResponse, Serializeable},
    http_status::HttpStatus,
    routes, HttpServer,
};
use std::net::TcpListener;

fn read_this(request: &HttpRequest) -> Result<HttpResponse<String, Serializeable>, HttpError> {
    HttpResponse::new(
        format!("This is the READINNGGG Path {}", request.path),
        vec![HttpHeader::ContentType("text/plain".to_string())],
        HttpStatus::Ok,
    )
}

fn post_this(request: &HttpRequest) -> Result<HttpResponse<String, Serializeable>, HttpError> {
    HttpResponse::new(
        format!("This is the UPDATINGG Path {}", request.path),
        vec![HttpHeader::ContentType("text/plain".to_string())],
        HttpStatus::Ok,
    )
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to socket");

    let handlers = routes!(
        Get, "/" -> read_this,
        Post, "/" -> post_this,
    );

    let server = HttpServer::new(listener, handlers);

    server.listen();
}
