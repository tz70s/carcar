//! This is a data source generator for testing flink stream processing
//! Author Tzu-Chiao Yeh @tz70s

/// This file works on benching by spawning multiple threads.

use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::{Receiver, channel};
use std::io::{self, BufRead};


/// The sended objects need to implement the bencher trait.
pub trait Bencher {
    fn generate() -> Self;
    fn render(&mut self, c: &::config::Config);
    fn serialized_to_string(&self) -> String;
}

/// The thread frame to each.
#[derive(Clone)]
struct ThreadFrame {
    id: u32,
    destination: String,
    stop_signal: bool,
}

impl ThreadFrame {
    fn new(id: u32, destination: String, stop_signal: bool) -> ThreadFrame {
        ThreadFrame {
            id: id,
            destination: destination,
            stop_signal: stop_signal
        }
    }
}

// Generate data payload along with different threads
fn fire<T: Bencher>(receiver: Receiver<bool>, c: ::config::Config, tf: ThreadFrame) {
    let mut stream = TcpStream::connect(tf.destination).unwrap();
    let mut bencher = T::generate();
    loop {
        bencher.render(&c);
        let _ = stream.write(bencher.serialized_to_string().as_bytes());
        // Try to receive the signal
        match receiver.try_recv() {
            Ok(stop_signal) => {
                if stop_signal {
                    println!("Terminated");
                    break;
                }
            },
            Err(_) => {} 
            // Keep going
        }
    }
}

// Run parallel of each tcp stream connections.
// Each stream will continously sending data to the destination.
fn bench_parallel(num_of_threads: u32, c: &::config::Config) {
    // The vector for recording spawning thread and associated join handlers
    let mut forks = vec![];
    let mut chan_of_each = vec![];
    // Cloning thread frame to each, by parsing config and each weight.
    // The content will contain:
    // 1. Target destination.
    // 2. Stop signal.
    for num in 0..num_of_threads {
        let (sender, receiver) = channel();
        // Create a new thread frame for the thread.
        let mut send_tf = ThreadFrame::new(num, c.destination.ip.to_owned() + ":" + &c.destination.port, false);
        sender.send(false);
        let mut recv_tf = send_tf.clone();
        let clone_c = c.clone();
        forks.push(thread::spawn(move || {
            fire::<::car::CarPayload>(receiver, clone_c, recv_tf);
        }));
        chan_of_each.push((sender, send_tf));
    }

    for frame in &chan_of_each {
        if !frame.1.stop_signal {
            println!("[{}] stream to - {}", frame.1.id, frame.1.destination);
        }
    }

    // Use stdin for terminate the spawnning threads.
    // TODO: makes the channel identified more verbose, not just an vector of integer.
    // TODO: make the stdin into a new TCP socket, CAREFULLY dealing with error here!
    // TODO: what if spawning a new thread here?
    let stdin = io::stdin();
    loop {
        let mut line = String::new();
        let _ = stdin.lock().read_line(&mut line);
        let which = line.trim().parse::<u32>().expect("invalid digits!");
        let which = which as usize;
        // Terminate, currently, makes the stop signal to true.
        chan_of_each[which].1.stop_signal = true;
        chan_of_each[which].0.send(true).unwrap();
        // Print the current running threads
        for frame in &chan_of_each {
            if !frame.1.stop_signal {
                println!("[{}] stream to - {}", frame.1.id, frame.1.destination);
            }
        }
    }
}

// Entry point of car module
pub fn bench(num_of_threads: u32, c: &::config::Config) {
    println!("Start sending traffic data into {}", c.destination.ip.to_owned() + ":" + &c.destination.port);
    bench_parallel(num_of_threads, c);
}
