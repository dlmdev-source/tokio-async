use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADRESS: &str = "localhost:1234";

fn main(){
    // connection
    println!("connecting to {}", ECHO_SERVER_ADRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADRESS) {
        // connected
        println!("connected to echo server {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        // write a hello world message
        let message = "Hello World! ";
        let _ = stream.write(message.as_bytes());
        let _ = stream.flush();
        println!("send: {}", message);

        // read the result
        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer);
        println!("received: {}", message)

    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER_ADRESS);
    }

}

