/// This is a data source generator for testing stream processing
/// Author Tzu-Chiao Yeh @tz70s

/// This file parse the config from toml :
/// 1. cars configuration
/// 2. target destination

use std::fs::File;
use std::io::prelude::*;

/// Open configuration file, pass the file name from argument
fn open_config(file_name: &str, mut contents: &mut String) {
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut contents).unwrap();
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub car: CarConfig,
}

#[derive(Deserialize, Clone)]
pub struct CarConfig {
    pub position: [u32; 4],
    pub speed: [u32; 2],
}

/// Parsing toml
pub fn parse_toml(file_name: &str) -> Config {
    let mut contents = String::new();
    open_config(file_name, &mut contents);
    let config: Config = ::toml::from_str(&contents[..]).unwrap();
    config
}

/// Do some unit testing for toml file parsing
#[cfg(test)]
mod test_config {
    #[test]
    fn test_open_config() {
        use super::*;
        let file_name = String::from("test/test_config.toml");
        let mut contents = String::new();
        open_config(&file_name[..], &mut contents);
    }

    #[test]
    fn test_toml_parsing() {
        use super::*;
        let file_name = String::from("test/test_config.toml");
        let config = parse_toml(&file_name[..]);
        assert_eq!(config.car.position[0], 0);
        assert_eq!(config.car.position[1], 4000);
        assert_eq!(config.car.position[2], 0);
        assert_eq!(config.car.position[3], 200);
        assert_eq!(config.car.speed[0], 60);
        assert_eq!(config.car.speed[1], 120);
    }
}
