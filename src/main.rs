use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::RwLock;
use models::ChatServerState;
mod models;
mod network;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    dotenvy::dotenv().ok();
    let ip = env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "7878".to_string());
    let address = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(&address).await?;
    println!("Asenkron Sohbet Sunucusu Başlatıldı: {}", address);
    let (tx, _rx) = broadcast::channel(16);
    let online_users = Arc::new(RwLock::new(Vec::new()));
    let state = Arc::new(ChatServerState { tx, online_users });
    loop {
        let (stream, _) = listener.accept().await?;
        let current_state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = network::handle_client(stream, current_state).await {
                eprintln!("İstemci hatası: {}", e);
            }
        });
    }
}