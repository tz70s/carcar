//! This is a data source generator for testing flink stream processing
//! Author Tzu-Chiao Yeh @tz70s

extern crate car_bench;

use std::env;
use std::process;

// Print the usage for wrong command line arguments
fn print_usage() {
    println!("
    Usage:
        cargo run [types] [number_of_payloads] of each [number_of_threads]
    The total number of payloads is [num_of_payloads] * [num_of_threads].
             ");
}

fn main() {
    // For parsing arguments.    
    let args: Vec<String> = env::args().collect();
    let mut num_of_payloads = 0;
    let mut num_of_threads = 0;
    let mut is_car_bench = true;
    match args.len() {
        2 => {
            match &args[1][..] {
                "fake" => {
                    is_car_bench = false;
                },
                _ => {
                    print_usage();
                    process::exit(1);
                },
            };
        },
        4 => {
            match &args[1][..] {
                "car" => {
                    is_car_bench = true;
                },
                _ => {
                    print_usage();
                    process::exit(1);
                },
            };
            match args[2].parse() {
                Ok(num) => {
                    num_of_payloads = num;
                },
                _ => {
                    print_usage();
                    process::exit(1);
                },
            };
            match args[3].parse() {
                Ok(num) => {
                    num_of_threads = num;
                },
                _ => {
                    print_usage();
                    process::exit(1);
                },
            }
        },
        _ => {
            print_usage();
            process::exit(1);
        }
    };

    if is_car_bench {
        car_bench::cars::car_bench(num_of_payloads, num_of_threads);
    } else {
        car_bench::fake_server::spawn();
    }
}
