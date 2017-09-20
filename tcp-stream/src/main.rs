//! This is a data source generator for testing flink stream processing
//! Author Tzu-Chiao Yeh @tz70s
extern crate rand;

use std::io::prelude::*;
use std::net::TcpStream;
use rand::distributions::{IndependentSample, Range};
use std::thread;
use std::env;
use std::process;

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
fn send_a_car() {
    let mut stream = TcpStream::connect("127.0.0.1:10024").unwrap();
    let car = CarPayload::generate();
    let _ = stream.write(car.serialized_to_string().as_bytes());
}

// Send cars in multi-threads with adjustable numbers
fn send_multi() {
    // nop
}

fn print_usage() {
    println!("
    Usage:
        cargo run [number_of_payloads] of each [number_of_threads]
    The total number of payloads is [num_of_payloads] * [num_of_threads].
             ");
}

fn main() {
    println!("Start sending traffic data into 127.0.0.1:10024");
    
    // For parsing arguments.    
    let args: Vec<String> = env::args().collect();
    let mut num_of_payloads = 0;
    let mut num_of_threads = 0;
    match args.len() {
        3 => {
            match args[1].parse() {
                Ok(num) => {
                    num_of_payloads = num;
                },
                _ => {},
            };
            match args[2].parse() {
                Ok(num) => {
                    num_of_threads = num;
                },
                _ => {},
            }
        },
        _ => {
            print_usage();
            process::exit(1);
        }
    };

    // The vector for recording spawning thread and associated join handlers
    let mut forks = vec![];
    // First, the number of payloads 
    for _ in 0..num_of_payloads {
        // Spawn threads 
        for _ in 0..num_of_threads {
            forks.push(thread::spawn(move || {
                send_a_car();
            }));
        }
    }
    // Joins
    for child in forks {
        // Wait each child to finish 
        let _ = child.join();
    }
}
