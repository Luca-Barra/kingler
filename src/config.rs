use serde::{Deserialize, Serialize};

use std::env;
use std::fs;
use std::io::ErrorKind::NotFound;

use crate::error::Error;

const BINARY_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Language used when printing pokemon names and descriptions.
    pub language: String,
    /// The probability to display a shiny pokemon with the random command
    pub shiny_rate: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            shiny_rate: 1.0 / 128.0,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let config_dir = match dirs::config_dir() {
            Some(dir) => dir.join(BINARY_NAME),
            None => {
                return Err(Error::Configuration(
                    "Failed to get config directory".to_string(),
                ))
            }
        };

        let config_file = config_dir.join("config.toml");
        let config = match fs::read_to_string(&config_file) {
            Ok(c) => toml::from_str(&c).expect("Failed to parse toml in configuration file"),

            // Create default config file if it doesn't exist
            Err(ref e) if e.kind() == NotFound => {
                let config = Config::default();
                let toml =
                    toml::to_string_pretty(&config).expect("Failed to convert config to toml");

                fs::create_dir_all(config_dir).expect("Failed to create config directory");
                fs::write(&config_file, toml).expect("Failed to write config file");
                config
            }
            Err(_) => {
                return Err(Error::Configuration(
                    "Failed to load configuration file".to_string(),
                ))
            }
        };

        Ok(config)
    }
}
