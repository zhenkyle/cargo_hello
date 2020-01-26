use std::io::prelude::*; // for std:io:Read etc.
use std::fs;
use std::net::{TcpListener, TcpStream};
use std::{thread, time};
// use hello::ThreadPool; // seems no needed

fn main() {
    let thread_pool = hello::ThreadPool::new(4);
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_client(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        let sleep_time = time::Duration::from_secs(5);
        thread::sleep(sleep_time);
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response=format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
