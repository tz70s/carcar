//! This is a fake listener for testing tcpstream
//! Author Tzu-Chiao Yeh, @tz70s

use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;

static mut NUMBER_OF_RECEIVED : u32 = 0;

fn handle_client(stream: &mut TcpStream) {
    loop {
        let mut buffer = String::new();
        let _ = stream.take(1024).read_to_string(&mut buffer);
        unsafe {
            NUMBER_OF_RECEIVED += 1;
            println!("read {} : {}", NUMBER_OF_RECEIVED, buffer);
        }
    }
}

pub fn spawn() {
    println!("Spawn a fake server for test...");
    let listener = TcpListener::bind(::ADDRESS).unwrap();
    println!("Listen at the {}", ::ADDRESS);
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            handle_client(&mut stream);
        });
    }
}
