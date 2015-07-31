extern crate thunderdome;
extern crate capnp;
extern crate capnpc;

use std::io::Read;
use std::io::{self, Cursor};
use std::collections::HashMap;
use thunderdome::graph::{Graph, Request};
use std::net::{TcpListener, TcpStream};
use std::thread;


use std::sync::mpsc::{channel, Sender};

mod thunderdome_capnp {
    include!("thunderdome_capnp.rs");
}

use thunderdome_capnp::graph;

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

fn handle_client(mut stream: TcpStream, tx: Sender<Request> ) {
    println!("connected.  opening local channel for graph comm and creating request context");

    // let (client_tx, client_rx) = channel();
    // let mut buffer = String::new();
    let mut buffer: Vec<u8> = Vec::new();
    loop {
        // read the header - 4 bytes?
        match  stream.read_to_end(&mut buffer) {
            Ok(0) => {
                println!("no bytes received bailing out");
                return ()
            }
            Ok(bytes) => {
                println!("ok - command received {}", bytes);
            },
            Err(_) => {
                println!("Error, leaving");
                return ();
            }
        };
    }

}
