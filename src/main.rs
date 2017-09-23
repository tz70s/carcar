/// This is a data source generator for testing flink stream processing
/// Author Tzu-Chiao Yeh @tz70s

extern crate carcar;
extern crate clap;

use clap::{Arg, App, SubCommand};
use std::str::FromStr;
use std::process;

/// Entry point
fn main() {
    // Parsing commands from clap.
    let matches = App::new("Carcar")
                        .version("0.3")
                        .author("Tzu-Chiao Yeh <su3g4284zo6y7@gmail.com>")
                        .about("Car-liked benching data generator for streaming testing")
                        .arg(Arg::with_name("concurrency")
                             .short("c")
                             .long("concurrency")
                             .help("Sets the level(number) of concurrent threads")
                             .takes_value(true))
                        .arg(Arg::with_name("debug")
                             .short("d")
                             .long("debug")
                             .help("Spawn the debug server")
                             .multiple(true))
                        .arg(Arg::with_name("log")
                             .short("l")
                             .long("log")
                             .help("Print the payload sent from carcar")
                             .multiple(true))
                        .arg(Arg::with_name("model")
                             .short("m")
                             .long("model")
                             .help("The file path for a specific model")
                             .takes_value(true))
                        .arg(Arg::with_name("dport")
                             .short("p")
                             .long("dport")
                             .help("The port which is the debug server listened to")
                             .takes_value(true))
                        .arg(Arg::with_name("dtime")
                             .short("t")
                             .long("dtime")
                             .help("The keep time of the debug server handling stream")
                             .takes_value(true))
                        .subcommand(SubCommand::with_name("list")
                                    .about("List the existed models"))
                        .get_matches();
    // Level of concurrency
    let concurrency = matches.value_of("concurrency").unwrap_or("1");
    let concurrency: u32 = FromStr::from_str(concurrency).unwrap();
    // Lists the existed models
    if let Some(_) = matches.subcommand_matches("list") {
        println!("single_road_model");
        process::exit(0);
    }
    // Model file, defualt is the single_road_model
    let model_file = matches.value_of("model").unwrap_or("model/single_road_model.toml"); 
    // Parse the configuration file
    let conf = carcar::config::parse_toml(model_file);
    // Checkout whether the logger is opened.
    let logger = match matches.occurrences_of("log") {
        0 => {
            false
        },
        _ => {
            true
        }
    };
    // If the debug mode is specified, spawn the fake server for testing
    match matches.occurrences_of("debug") {
        0 => {
            carcar::bench::bench(concurrency, &conf);
        },
        _ => {
            // Check whether the debug server is on, or default is 10023
            let dport = matches.value_of("dport").unwrap_or("10023");
            // The keep time parsed from args
            let dtime = matches.value_of("dtime").unwrap_or("10");
            let dtime = dtime.parse::<u64>().unwrap();
            carcar::debug::spawn(concurrency, dport, logger, dtime); 
        }
    }
}
