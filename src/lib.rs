//! This is a data source generator for testing flink stream processing
//! Author Tzu-Chiao Yeh @tz70s

#[macro_use]
extern crate toml;
#[macro_use]
extern crate serde_derive;

pub mod car;
pub mod fake_server;
pub mod config;

static ADDRESS: &'static str = "127.0.0.1:10023";
