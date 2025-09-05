use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub base_url: String,
    pub pwm: PwmConfig,
    pub pins: PinsConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PwmConfig {
    pub period_nanos: u64,
    pub sleep_nanos: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PinsConfig {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub general: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:3000".to_string(),
            pwm: PwmConfig {
                period_nanos: 1_000_000, // 1 millisecond
                sleep_nanos: 2,
            },
            pins: PinsConfig {
                red: 23,
                green: 24,
                blue: 27,
                general: 22,
            },
        }
    }
}

impl Config {
    pub fn open() -> Result<Self, String> {
        if !fs::exists("config.toml").map_err(|_| "config.toml not found")? {
            let config = Self::default();
            fs::write("config.toml", toml::to_string(&config).unwrap()).unwrap();
            return Ok(config);
        }

        let content = fs::read_to_string("config.toml")
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        toml::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))
    }
}
