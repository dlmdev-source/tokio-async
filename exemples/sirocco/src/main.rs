use std::net::TcpListener;

// constants
const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:5678";

fn main() {
    // starting
    println!("sirocco starting {}", SIROCCO_SERVER_ADDRESS);

    // bind 
    let listener = TcpListener::bind(SIROCCO_SERVER_ADDRESS).unwrap();

    //start
    println!("sirocco listening {}", SIROCCO_SERVER_ADDRESS);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("connection established!")
    }
}
