/// This is a data source generator for testing stream processing
/// Author Tzu-Chiao Yeh @tz70s

/// This file works on benching by spawning multiple threads.

use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::{Receiver, channel};
use std::io::{self, BufRead};
use std::process;

/// The sended objects need to implement the bencher trait.
pub trait Bencher {
    fn generate() -> Self;
    fn render(&mut self, c: &::config::Config);
    fn serialized_to_string(&self) -> String;
}

/// The about function which will print at the top of screen.
fn about_fn() {
    process::Command::new("clear").status().unwrap();
    println!("Carcar 0.3");
    println!("Tzu-Chiao Yeh <su3g4284zo6y7@gmail.com>");
    println!("Car-liked benching data generator for stream processing");
    println!("The interactive commands:");
    println!("[1] migrate <thread_number> <destination>");
    println!("[2] stop <thread_number>");
    println!("=====================================================================");
}

/// The thread frame to each.
#[derive(Clone)]
struct ThreadFrame {
    id: u32,
    destination: String,
    stop_signal: bool,
}

/// Generate new thread frame
impl ThreadFrame {
    fn new(id: u32, destination: &str, stop_signal: bool) -> ThreadFrame {
        ThreadFrame {
            id: id,
            destination: destination.to_owned(),
            stop_signal: stop_signal
        }
    }
}

/// Generate data payload along with different threads
fn fire<T: Bencher>(receiver: Receiver<ThreadFrame>, c: ::config::Config, tf: ThreadFrame) {
    let mut stream = TcpStream::connect(tf.destination).unwrap();
    let mut bencher = T::generate();
    loop {
        bencher.render(&c);
        let _ = stream.write(bencher.serialized_to_string().as_bytes());
        // Try to receive the signal
        match receiver.try_recv() {
            Ok(ntf) => {
                if ntf.stop_signal {
                    break;
                }
                stream = TcpStream::connect(ntf.destination).unwrap();
            },
            Err(_) => {} 
            // Keep going
        }
    }
}

/// Run parallel of each tcp stream connections.
/// Each stream will continously sending data to the destination.
fn bench_parallel(num_of_threads: u32, c: &::config::Config, dst: &str) {
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
        let send_tf = ThreadFrame::new(num, dst, false);
        let recv_tf = send_tf.clone();
        let clone_c = c.clone();
        forks.push(thread::spawn(move || {
            fire::<::car::CarPayload>(receiver, clone_c, recv_tf);
        }));
        chan_of_each.push((sender, send_tf));
    }
    about_fn();
    for frame in &chan_of_each {
        if !frame.1.stop_signal {
            println!("[{}] stream to - {}", frame.1.id, frame.1.destination);
        }
    }
    println!("=====================================================================");
    // Use stdin for terminate the spawnning threads.
    // TODO: makes the channel identified more verbose, not just an vector of integer.
    // TODO: make the stdin into a new TCP socket, CAREFULLY dealing with error here!
    // TODO: what if spawning a new thread here?
    let stdin = io::stdin();
    loop {
        let mut line = String::new();
        let _ = stdin.lock().read_line(&mut line);
        let mut command = line.split_whitespace();
        let which;
        match command.next().unwrap() {
            "migrate" => {
                // To change the target destination
                which = match command.next() {
                    Some(s) => {
                        s.parse::<i32>().expect("invalid digits")
                    },
                    None => -1
                };
                if which >= 0 {
                    let which = which as usize;
                    // Match the new target destination
                    match command.next() {
                        Some(s) => {
                            chan_of_each[which].1.destination = s.to_owned();
                            chan_of_each[which].0.send(chan_of_each[which].1.clone()).expect("can't send the thread frame");
                        },
                        None => {}
                    }
                }
            },
            "stop" => {
                // Stop a thread
                which = match command.next(){
                    Some(s) => {
                        s.parse::<i32>().expect("invalid digits")
                    },
                    None => -1
                };
                if which >= 0 {
                    let which = which as usize;
                    // Terminate, makes the stop signal to true
                    chan_of_each[which].1.stop_signal = true;
                    // Sender sends true to the target thread
                    chan_of_each[which].0.send(chan_of_each[which].1.clone()).expect("can't send the thread frame");
                }
            },
            _ => {
                // nop, drop to the next iteration
            }
        };
        about_fn();
        // Print the current running threads
        for frame in &chan_of_each {
            if !frame.1.stop_signal {
                println!("[{}] stream to - {}", frame.1.id, frame.1.destination);
            }
        }
        println!("=====================================================================");
    }
}

/// Entry point of benchmark module
pub fn bench(num_of_threads: u32, c: &::config::Config, dst: &str) {
    println!("Start sending traffic data into {}", dst);
    bench_parallel(num_of_threads, c, dst);
}
