extern crate conduit;
extern crate conduit_router;
extern crate civet;
extern crate thunderdome;

use conduit::{Request, Response};
use conduit_router::{RouteBuilder, RequestParams};
use civet::{Config, response, Server};
use std::sync::mpsc::channel;
use std::io::{self, Cursor};
use std::collections::HashMap;


fn hello(_req: &mut Request) -> io::Result<Response> {
    Ok(response(200, HashMap::new(), "Thunderdome graph server, Hello World!".as_bytes()))
}

fn main() {
    println!("Thunderdome Graph Server starting up...");

    let mut router = RouteBuilder::new();
    router.get("/", hello);

    let server = Server::start(Config { port: 8888, threads: 1 }, router);

    // Preventing process exit.
    let (_tx, rx) = channel::<()>();
    rx.recv().unwrap();

    println!("Thunderdome Graph Server shutting down");
}
