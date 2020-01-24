//use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(stream: TcpStream) {
    println!("Got a connection!");
}
