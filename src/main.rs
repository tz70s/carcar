//! This is a data source generator for testing flink stream processing
//! Author Tzu-Chiao Yeh @tz70s

extern crate carcar;
#[macro_use]
extern crate toml;
extern crate serde;

use std::env;
use std::process;

// Print the usage for wrong command line arguments
fn helper() {
    println!("
    Usage:
        cargo run [car/fake] [number_of_rounds] [number_of_threads] 
    The total number of payloads is [num_of_rounds] * [num_of_threads].
             ");
    process::exit(1);
}

fn main() {
    // For parsing arguments.    
    let args: Vec<String> = env::args().collect();
    let mut num_of_rounds = 0;
    let mut num_of_threads = 0;
    
    // Used for check is car bench or fake server
    let mut is_car_bench = true;
    match args.len() {
        3 => {
            match &args[1][..] {
                "fake" => {
                    is_car_bench = false;
                },
                _ => {
                    helper();
                },
            };
            match args[2].parse() {
                Ok(num) => {
                    num_of_threads = num;
                },
                _ => {
                    helper();
                }
            }
        },
        4 => {
            match &args[1][..] {
                "car" => {
                    is_car_bench = true;
                },
                _ => {
                    helper();
                },
            };
            match args[2].parse() {
                Ok(num) => {
                    num_of_rounds = num;
                },
                _ => {
                    helper();
                },
            };
            match args[3].parse() {
                Ok(num) => {
                    num_of_threads = num;
                },
                _ => {
                    helper();
                },
            }
        },
        _ => {
            helper();
        }
    };

    if is_car_bench {
        carcar::bench::bench(num_of_rounds, num_of_threads);
    } else {
        carcar::fake_server::spawn(num_of_threads);
    }
}
