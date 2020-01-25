use std::io::prelude::*; // for std:io:Read etc.
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
// My version also works
/*
fn handle_client(mut stream: TcpStream) {
    let buffer = &mut [0; 512];
    stream.read(buffer).unwrap();
    println!("{}",String::from_utf8_lossy(buffer));
}
 */

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("{}",String::from_utf8_lossy(&buffer[..]));
    if String::from_utf8_lossy(&buffer[..]).starts_with("GET / HTTP/1.1") {
        stream.write(b"HTTP/1.1 200 OK\nContent-Type: text/html; charset=UTF-8\n\n<html><body><h1>Hello world!</h1></body></html>\n").unwrap();
    } else {
        stream.write(b"HTTP/1.1 404 Not Found\nContent-Type: text/html; charset=UTF-8\n\n<html><body><h1>Not Found!</h1></body></html>\n").unwrap();
    }
}
