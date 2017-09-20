//! This is a fake listener for testing tcpstream
//! Author Tzu-Chiao Yeh, @tz70s

use std::net::{TcpListener, TcpStream};
use std::io::Read;

static mut NUMBER_OF_RECEIVED : u32 = 0;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = String::new();
    let _ = stream.read_to_string(&mut buffer);
    unsafe {
        NUMBER_OF_RECEIVED += 1;
        println!("read {} : {}", NUMBER_OF_RECEIVED, buffer);
    }
}

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:10024").unwrap();
    println!("Listen at the 127.0.0.1:10024");
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
