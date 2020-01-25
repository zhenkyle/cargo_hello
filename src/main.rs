use std::io::prelude::*; // for std:io:Read etc.
use std::fs::File;
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
    let mut file = File::open("200.html").unwrap();
    let mut text_200 = String::new();
    file.read_to_string(&mut text_200).unwrap();
    file = File::open("404.html").unwrap();
    let mut text_404 = String::new();
    file.read_to_string(&mut text_404).unwrap();

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("{}",String::from_utf8_lossy(&buffer[..]));
    if String::from_utf8_lossy(&buffer[..]).starts_with("GET / HTTP/1.1") {
        stream.write(b"HTTP/1.1 200 OK\nContent-Type: text/html; charset=UTF-8\n\n").unwrap();
        stream.write(text_200.as_bytes()).unwrap();
    } else {
        stream.write(b"HTTP/1.1 404 Not Found\nContent-Type: text/html; charset=UTF-8\n\n").unwrap();
        stream.write(text_404.as_bytes()).unwrap();
    }
}
