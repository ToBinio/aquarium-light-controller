use std::{thread, time::Duration};

use crate::config::Config;

#[derive(Default, Debug)]
pub struct LightBrightness {
    red: u8,
    blue: u8,
    green: u8,
    general: u8,
}

impl LightBrightness {
    pub fn new(red: u8, blue: u8, green: u8, general: u8) -> Self {
        Self {
            red,
            blue,
            green,
            general,
        }
    }
}

pub fn spawn(config: &Config) -> std::sync::mpsc::Sender<LightBrightness> {
    let (sender, receiver) = std::sync::mpsc::channel::<LightBrightness>();

    let pwm_period_nanos = config.pwm.period_nanos;
    thread::spawn(move || {
        let period_duration: Duration = Duration::from_nanos(pwm_period_nanos);

        let mut brightness = LightBrightness::default();
        let mut period_start = std::time::Instant::now();

        loop {
            if let Ok(new_brightness) = receiver.try_recv() {
                brightness = new_brightness;
            }

            let passed_part: u8 = ((period_start.elapsed().as_nanos() as f32
                / period_duration.as_nanos() as f32)
                * u8::MAX as f32) as u8;

            println!(
                " r{} g{} b{} g{}",
                brightness.red > passed_part,
                brightness.green > passed_part,
                brightness.blue > passed_part,
                brightness.general > passed_part
            );

            if period_start.elapsed() > period_duration {
                period_start = std::time::Instant::now();
            }

            thread::yield_now();
        }
    });

    sender
}
