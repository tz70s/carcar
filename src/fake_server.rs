//! This is a fake listener for testing tcpstream
//! Author Tzu-Chiao Yeh, @tz70s

use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;
use std::time::{Duration, SystemTime};
use std::sync::mpsc::{Receiver, channel};

fn handle_client(stream: &mut TcpStream) -> i32 {
    let mut num_of_receive = 0;
    let now = SystemTime::now();
    
    loop {
        let mut buffer = String::new();
        let _ = stream.take(1048576).read_to_string(&mut buffer);
        num_of_receive += 1;
        if now.elapsed().unwrap().as_secs() == 10 {
            break;
        }
    }
    num_of_receive
}

pub fn spawn() {
    println!("Spawn a fake server for test...");
    let listener = TcpListener::bind(::ADDRESS).unwrap();
    println!("Listen at the {}", ::ADDRESS);
    let mut total = 0;
    let mut break_count = 0;
    let mut receiver_vec = vec![];
    for stream in listener.incoming() {
        break_count += 1;
        let (sender, receiver) = channel();
        receiver_vec.push(receiver);
        thread::spawn(move || {
            let mut stream = stream.unwrap();
            let num_of_receive = handle_client(&mut stream);
            sender.send(num_of_receive);
        });
        if break_count == 9 {
            break;
        }
    }
    
    for receiver in receiver_vec {
        total += receiver.recv().unwrap();
    }

    let bandwidth = total / 10;
    println!("The receiving bandwidth is : {} Mbps", bandwidth * 8);
}
