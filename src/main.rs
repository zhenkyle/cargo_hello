use std::io::prelude::*; // for std:io:Read etc.
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}

fn handle_client(mut stream: TcpStream) {
    let buffer = &mut [0; 512];
    stream.read(buffer).unwrap();
    println!("{}",String::from_utf8_lossy(buffer));
}
