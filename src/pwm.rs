use std::{thread, time::Duration};

use rppal::gpio::{Gpio, Level, OutputPin};

use crate::config::Config;

#[derive(Default, Debug)]
pub struct LightBrightness {
    red: u8,
    green: u8,
    blue: u8,
    general: u8,
}

impl LightBrightness {
    pub fn new(red: u8, green: u8, blue: u8, general: u8) -> Self {
        Self {
            red,
            green,
            blue,
            general,
        }
    }
}

pub struct Pins {
    red: OutputPin,
    blue: OutputPin,
    green: OutputPin,
    general: OutputPin,
}

impl Pins {
    pub fn from_config(config: &Config, gpio: &Option<Gpio>) -> Option<Self> {
        match gpio {
            Some(gpio) => Some(Self {
                red: gpio.get(config.pins.red).unwrap().into_output(),
                blue: gpio.get(config.pins.blue).unwrap().into_output(),
                green: gpio.get(config.pins.green).unwrap().into_output(),
                general: gpio.get(config.pins.general).unwrap().into_output(),
            }),
            None => None,
        }
    }
}

pub fn spawn(config: &Config) -> std::sync::mpsc::Sender<LightBrightness> {
    let (sender, receiver) = std::sync::mpsc::channel::<LightBrightness>();

    let config = config.clone();
    thread::spawn(move || {
        let period_duration: Duration = Duration::from_nanos(config.pwm.period_nanos);

        let gpio = Gpio::new().ok();
        let mut pins = Pins::from_config(&config, &gpio);

        let mut brightness = LightBrightness::default();
        let mut period_start = std::time::Instant::now();

        loop {
            if let Some(new_brightness) = receiver.try_recv().ok() {
                brightness = new_brightness;
            }

            let passed_part: u8 = ((period_start.elapsed().as_nanos() as f32
                / period_duration.as_nanos() as f32)
                * u8::MAX as f32) as u8;

            if let Some(pins) = &mut pins {
                //TODO - only set if change
                if brightness.red > passed_part {
                    pins.red.set_high();
                } else {
                    pins.red.set_low();
                }

                if brightness.blue > passed_part {
                    pins.blue.set_high();
                } else {
                    pins.blue.set_low();
                }

                if brightness.green > passed_part {
                    pins.green.set_high();
                } else {
                    pins.green.set_low();
                }

                if brightness.general > passed_part {
                    pins.general.set_high();
                } else {
                    pins.general.set_low();
                }
            } else {
                println!(
                    " r{} g{} b{} g{}",
                    brightness.red > passed_part,
                    brightness.green > passed_part,
                    brightness.blue > passed_part,
                    brightness.general > passed_part
                );
            }

            if period_start.elapsed() > period_duration {
                period_start = std::time::Instant::now();
            }

            thread::sleep(std::time::Duration::from_nanos(config.pwm.sleep_nanos));
            thread::yield_now();
        }
    });

    sender
}
