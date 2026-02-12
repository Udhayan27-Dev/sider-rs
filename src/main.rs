use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
mod resp;
mod resp_result;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6380").await?;
    println!("Listening for a Connection....");
    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("Client Address{addr}");
                tokio::spawn(handle_connection(stream));
            }
            Err(e) => {
                println!("Error:{}", e);
                continue;
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer).await {
            Ok(size) if size != 0 => {
                let response = "+PONG\r\n";
                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("Error writing to socket : {}", e);
                }
            }
            Ok(_) => {
                println!("Connection Closed");
                return;
            }
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        }
    }
}
