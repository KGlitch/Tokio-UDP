use tokio::net::UdpSocket;
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let server_addr = "127.0.0.1:4999";
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(server_addr).await?;
    
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        socket.send(line.as_bytes()).await?;
        println!("Message sent to server: {}", line);
    }
    
    Ok(())
}