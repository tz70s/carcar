//! This is a data source generator for testing flink stream processing
//! Author Tzu-Chiao Yeh @tz70s
extern crate rand;

use std::io::prelude::*;
use std::net::TcpStream;
use rand::distributions::{IndependentSample, Range};

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
fn main() {
    println!("Hello, world!");
    let mut stream = TcpStream::connect("127.0.0.1:10024").unwrap();
    let car = CarPayload::generate();
    let _ = stream.write(car.serialized_to_string().as_bytes());
}
