use std::{
    thread::{self},
    time::Duration,
};

use crate::{config::Config, pwm::LightBrightness};

pub mod config;
pub mod pwm;

fn main() {
    let config = Config::open().unwrap();

    let sender = pwm::spawn(&config);

    let mut brightness: u8 = 0;
    let mut direction: bool = true;
    loop {
        if direction {
            brightness += 1;
        } else {
            brightness -= 1;
        }

        if brightness == u8::MAX {
            direction = false;
        } else if brightness == 0 {
            direction = true;
        }

        sender
            .send(LightBrightness::new(
                brightness, brightness, brightness, brightness,
            ))
            .unwrap();

        thread::sleep(Duration::from_millis(20));
    }
}
