use std::io::prelude::*; // for std:io:Read etc.
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let buffer = &mut [0; 512];
    stream.read(buffer)?;
    println!("{}",String::from_utf8_lossy(buffer));
    Ok(())
}
