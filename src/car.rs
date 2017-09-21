//! This is a data source generator for testing flink stream processing
//! Author Tzu-Chiao Yeh @tz70s
extern crate rand;

use std::io::prelude::*;
use std::net::TcpStream;
use self::rand::distributions::{IndependentSample, Range};
use std::thread;
use std::sync::mpsc::{Receiver, channel};
use std::io::{self, BufRead};

// Payload of an Car information
#[derive(Debug)]
struct CarPayload {
    section : u32,
    position: (u32, u32),
    speed: u32,
    // Additional information?
}

impl CarPayload {
    fn generate() -> CarPayload {
        let mut rng = rand::thread_rng();
        // Section range from 1-4
        let sections_range = Range::new(1, 4);
        // Position range from ([100-1000])
        let east_west_range = Range::new(100, 1000);
        let north_south_range = Range::new(100, 150);
        // Speed range from 60-110
        let speed_range = Range::new(60, 110);
        let sec = sections_range.ind_sample(&mut rng);

        CarPayload {
           section: sec,
           position: (north_south_range.ind_sample(&mut rng), 
                      east_west_range.ind_sample(&mut rng) + sec * 1000),
           speed: speed_range.ind_sample(&mut rng),
        }
    }
    // Serialized to string
    fn serialized_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

// Generate data payload along with different threads
fn send_a_car(num_of_rounds: u32, receiver: Receiver<bool>) {
    let mut stream = TcpStream::connect(::ADDRESS).unwrap();
    let mut count: u32 = 0;
    loop {
        let car = CarPayload::generate();
        let _ = stream.write(car.serialized_to_string().as_bytes());
        count += 1;
        if count == num_of_rounds {
            break;
        }
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
// TODO: Optimization ( Object creation? )
fn bench_parallel(num_of_rounds: u32, num_of_threads: u32) {
    // The vector for recording spawning thread and associated join handlers
    let mut forks = vec![];
    let mut chan_of_each = vec![];
    for _ in 0..num_of_threads {
        let (sender, receiver) = channel();
        sender.send(false);
        forks.push(thread::spawn(move || {
            send_a_car(num_of_rounds, receiver);
        }));
        chan_of_each.push(sender);
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
        chan_of_each[which].send(true);
    }
}

// Entry point of car module
pub fn bench(num_of_rounds: u32, num_of_threads: u32) {
    println!("Start sending traffic data into {}", ::ADDRESS);
    bench_parallel(num_of_rounds, num_of_threads);
}
