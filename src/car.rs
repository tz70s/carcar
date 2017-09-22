//! This is a data source generator for testing flink stream processing
//! Author Tzu-Chiao Yeh @tz70s
extern crate rand;

use self::rand::distributions::{IndependentSample, Range};

// Payload of an Car information
#[derive(Debug)]
pub struct CarPayload {
    position: (u32, u32),
    speed: u32,
    // Additional information?
}

impl ::bench::Bencher for CarPayload {
    fn generate() -> CarPayload {
        // Just generate the object
        CarPayload {
           position: (0, 0),
           speed: 0,
        }
    }

    // Render a car payload
    fn render(&mut self, c: &::config::Config) {
        let mut rng = rand::thread_rng();
        // Position range from [east, west, north, south]
        let east_west_range = Range::new(c.car.position[0], c.car.position[1]);
        let north_south_range = Range::new(c.car.position[2], c.car.position[3]);
        // Speed range
        let speed_range = Range::new(c.car.speed[0], c.car.speed[1]);
        self.position = (east_west_range.ind_sample(&mut rng),
                         north_south_range.ind_sample(&mut rng));
        self.speed = speed_range.ind_sample(&mut rng);
    }

    // Serialized to string
    fn serialized_to_string(&self) -> String {
        format!("{:?}", self)
    }
}
