use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::sync::Arc;
use crate::models::{ChatServerState, UserProfile};

pub async fn handle_client(mut stream: TcpStream, state: Arc<ChatServerState>) -> tokio::io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("Yeni bağlantı adresi: {}", peer_addr);
    let new_user = UserProfile {
        username: format!("Misafir_{}", peer_addr.port()),
        peer_addr: peer_addr.to_string(),
    };
    {
        let mut users = state.online_users.write().await;
        users.push(new_user);
        println!("Şu anki aktif kullanıcı sayısı: {}", users.len());
    }
    let (reader, mut writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);
    let mut rx = state.tx.subscribe();
    let tx = state.tx.clone();
    let _ = writer.write_all(b"--- CrabChat Sohbet Odasina Hos Geldiniz ---\r\n\r\n").await;
    let mut line = String::new();
    loop {
        tokio::select! {
            result = buf_reader.read_line(&mut line) => {
                match result {
                    Ok(0) => {
                        {
                            let mut users = state.online_users.write().await;
                            users.retain(|u| u.peer_addr != peer_addr.to_string());
                            println!("Biri çıktı. Kalan aktif kullanıcı sayısı: {}", users.len());
                        }
                        return Ok(());
                    }
                    Ok(_) => {
                        let msg = line.trim().to_string();
                        if !msg.is_empty() {
                            let formatted_msg = format!("[{}]: {}", peer_addr, msg);
                            let _ = tx.send(formatted_msg);
                        }
                        line.clear();
                    }
                    Err(e) => {
                        eprintln!("Okuma hatası: {}", e);
                        line.clear();
                        continue;
                    }
                }
            }
            msg_result = rx.recv() => {
                if let Ok(msg) = msg_result {
                    let client_msg = format!("{}\r\n", msg);
                    if let Err(_) = writer.write_all(client_msg.as_bytes()).await {
                        return Ok(());
                    }
                }
            }
        }
    }
}