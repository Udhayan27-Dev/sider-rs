use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6380").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_connection(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 512];
    loop {
        let n = match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };

        let request = String::from_utf8_lossy(&buffer[..n]);
        let response = if request.contains("PING") {
            "+PONG\r\n"
        } else {
            "+OK\r\n"
        };
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
