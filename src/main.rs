extern crate conduit;
extern crate conduit_router;
extern crate civet;

use conduit::{Request, Response};
use conduit_router::{RouteBuilder, RequestParams};
use civet::{Config, response, Server};
use std::sync::mpsc::channel;

fn main() {
    println!("Thunderdome Graph Server starting up...");

    let mut router = RouteBuilder::new();
    let server = Server::start(Config { port: 8888, threads: 1 }, router);

    // Preventing process exit.
    let (_tx, rx) = channel::<()>();
    rx.recv().unwrap();

    println!("Thunderdome Graph Server shutting down");
}
