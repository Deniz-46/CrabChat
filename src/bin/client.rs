use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:7878").await?;
    println!("--- CrabChat Sunucusuna Baglanildi! ---");
    let (reader, mut writer) = stream.into_split();
    let mut buf_reader = BufReader::new(reader);
    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut keyboard_line = String::new();
    let mut server_line = String::new();

    loop {
        tokio::select! {
            server_result = buf_reader.read_line(&mut server_line) => {
                if server_result? == 0 {
                    println!("Sunucu baglantisi koptu.");
                    break;
                }
                print!("{}", server_line);
                server_line.clear();
            }
            keyboard_result = stdin.read_line(&mut keyboard_line) => {
                if keyboard_result? == 0 { break; }
                let msg = format!("{}\r\n", keyboard_line.trim());
                writer.write_all(msg.as_bytes()).await?;
                keyboard_line.clear();
            }
        }
    }
    Ok(())
}