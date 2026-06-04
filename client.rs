use std::io::{self, BufRead, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Yankı sunucusuna bağlandı.");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        writeln!(stream, "{}", line)?;
        let mut response = [0u8; 1024];
        let n = std::io::Read::read(&mut stream, &mut response)?;
        print!("Echo: {}", String::from_utf8_lossy(&response[..n]));
    }
    Ok(())
}