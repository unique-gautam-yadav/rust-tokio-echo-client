use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const ECHO_SERVER_ADDRES: &str = "localhost:1234";
#[tokio::main]
async fn main() {
    // connection
    println!("Connecting to {}", ECHO_SERVER_ADDRES);
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRES).await {
        println!(
            "connected to echo server {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port(),
        );

        // Write hello world msg
        let message = "Hello World!";
        let _ = stream.write_all(message.as_bytes()).await;
        println!("sent: {}", message);

        // read the result
        let mut buf = [0; 1024];
        let len = stream.read(&mut buf).await.unwrap();

        let message = String::from_utf8_lossy(&buf);
        println!("received: {}, len: {}", message, len);
    } else {
        println!("Failed to connect to echo server {}", ECHO_SERVER_ADDRES);
    }
}
