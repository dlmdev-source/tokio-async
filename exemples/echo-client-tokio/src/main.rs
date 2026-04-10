use std::io::prelude::*;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;

const ECHO_SERVER_ADRESS: &str = "localhost:1234";

#[tokio::main]
async fn main(){
    // connection
    println!("connecting to {}", ECHO_SERVER_ADRESS);
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADRESS).await {
        // connected
        println!("connected to echo server {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        // write a hello world message
        let message = "Hello World! ";
        let _ = stream.write_all(message.as_bytes()).await;
        println!("send: {}", message);

        // read the result
        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer);
        println!("received: {}", message)

    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER_ADRESS);
    }

}
