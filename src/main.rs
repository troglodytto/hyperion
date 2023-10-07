#![deny(clippy::pedantic)]
#![deny(clippy::missing_panics_doc)]

use hyperion::http::{response::Response, route, router::routes, server::Server};
use serde::Serialize;
use std::net::TcpListener;

#[derive(Serialize)]
struct Testing<'a> {
    name: &'a str,
    place: u8,
    vectors: Vec<String>,
}

#[route]
fn application(request: &Request) -> Response<String> {
    Response::Ok().json(Testing {
        name: "Hello",
        place: 8,
        vectors: vec![],
    })
}

#[route]
fn another_application(request: &Request) -> Response<&str> {
    Response::Ok().text("JSONING HARD")
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to socket");

    let router = routes!(
        Get "/" -> application,
        Get "/another" -> another_application,
    );

    let server = Server::new(listener, router);

    server.listen();
}
