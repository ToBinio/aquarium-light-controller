use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub pwm: PwmConfig,
}

#[derive(Serialize, Deserialize)]
pub struct PwmConfig {
    pub period_nanos: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pwm: PwmConfig {
                period_nanos: 10_000, // 10 microsecond
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
