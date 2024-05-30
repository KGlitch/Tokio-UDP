use tokio::net::UdpSocket;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:4999").await?;
    println!("Server is listening on port 4999");
    
    let mut buf = vec![0u8; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let message = String::from_utf8_lossy(&buf[..len]);
        println!("Received message from {}: {}", addr, message);
    }
}