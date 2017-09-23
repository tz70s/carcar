/// This work is a data source generator for testing stream processing
/// Author Tzu-Chiao Yeh, @tz70s
/// In this <debug.rs> file, spawn a fake server for receiving data and counting the throughput, in debug mode.

use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;
use std::time::{SystemTime};
use std::sync::mpsc::{channel};

/// Handling client of each stream
fn handle_client(stream: &mut TcpStream, logger: bool) -> i32 {
    let mut num_of_receive = 0;
    let now = SystemTime::now();
    // Trim 1MB stream and iterate to the next step, due to reveal the stream content.
    loop {
        let mut buffer = String::new();
        let _ = stream.take(1048576).read_to_string(&mut buffer);
        if logger {
            println!("{}", buffer);
        }
        num_of_receive += 1;
        if now.elapsed().unwrap().as_secs() == 10 {
            break;
        }
    }
    num_of_receive
}

/// Spawn a fake server
pub fn spawn(num_of_threads: u32, logger: bool) {
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
            let num_of_receive = handle_client(&mut stream, logger);
            sender.send(num_of_receive).unwrap();
        });
        if break_count == num_of_threads {
            break;
        }
    }
    // Accept the final counting size sent from each threads.
    for receiver in receiver_vec {
        total += receiver.recv().unwrap();
    }
    let bandwidth = total / 10;
    println!("The receiving bandwidth is : {} Mbps", bandwidth * 8);
}
