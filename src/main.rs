extern crate conduit;
extern crate conduit_router;
extern crate civet;
extern crate thunderdome;

use std::io::{self, Cursor};
use std::collections::HashMap;
use thunderdome::graph::{Graph, Request};
use std::cell::RefCell;
use std::sync::Arc;
use std::net::{TcpListener, TcpStream};
use std::thread;

use std::sync::mpsc::{channel, Sender};

fn main() {
    println!("Thunderdome Graph Database Server starting up...");

    // create an instance of the graph, wrap in RefCell / rc and let it do it's thing
    let g = Graph::new();
    println!("Graph up");

    let tx = g.execution_channel();

    println!("open execution channel");

    let listener = TcpListener::bind("127.0.0.1:6000").unwrap();

    println!("Socket bound");

    for stream in listener.incoming() {
        println!("connection established, spawning new thread");
        let new_tx = tx.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream, new_tx)
                    });
            },
            Err(e) => {}
        }
    }


    println!("Thunderdome Graph Database Server shutting down, goodbye.");
}

fn handle_client(stream: TcpStream, tx: Sender<Request> ) {
    println!("connected.  opening local channel for graph comm and creating request context");




}
