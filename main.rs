use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut stream: TcpStream) -> tokio::io::Result<()> {
    let mut buffer = [0u8; 1024];
    let peer_addr = stream.peer_addr()?;
    println!("New connection from: {}", peer_addr);
    loop {
        let bytes_read = stream.read(&mut buffer).await?;
        if bytes_read == 0 {
            println!("Client {} disconnected", peer_addr);
            return Ok(());
        }
        stream.write_all(&buffer[..bytes_read]).await?;
    }
}
#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    dotenvy::from_filename("sunucubilgileri.env").ok();
    let ip = env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "7878".to_string());
    let address = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(&address).await?;
    println!("Yankı sunucusu {} portunu dinliyor.", address);
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}