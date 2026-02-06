use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6380").unwrap();
    println!("Listening for a Connection....");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted a new connection");
                handle_connection(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream){
    let mut buffer = [0; 512];
    loop{
        match stream.read(&mut buffer){
            Ok(size) if size != 0 =>{
                let response = "+PONG\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();                
            }
            Ok(_) => {
                println!("Connection Closed");
                break;
            }
            Err(e) => {
                println!("Error: {}",e);
                break;
            }
        }
    }
}