use std::{thread, time::Duration};

use rppal::gpio::{Gpio, OutputPin};

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
        gpio.as_ref().map(|gpio| Self {
            red: gpio.get(config.pins.red).unwrap().into_output(),
            blue: gpio.get(config.pins.blue).unwrap().into_output(),
            green: gpio.get(config.pins.green).unwrap().into_output(),
            general: gpio.get(config.pins.general).unwrap().into_output(),
        })
    }

    pub fn set_status(pin: &mut OutputPin, active: bool) {
        let current_active = pin.is_set_high();

        if current_active == active {
            return;
        }

        if active {
            pin.set_high();
        } else {
            pin.set_low();
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
            if let Ok(new_brightness) = receiver.try_recv() {
                brightness = new_brightness;

                println!(
                    "new settings red-{} green-{} blue-{} all-{}",
                    brightness.red, brightness.green, brightness.blue, brightness.general
                );
            }

            let passed_part: u8 = ((period_start.elapsed().as_nanos() as f32
                / period_duration.as_nanos() as f32)
                * u8::MAX as f32) as u8;

            if let Some(pins) = &mut pins {
                Pins::set_status(&mut pins.red, brightness.red > passed_part);
                Pins::set_status(&mut pins.green, brightness.green > passed_part);
                Pins::set_status(&mut pins.blue, brightness.blue > passed_part);
                Pins::set_status(&mut pins.general, brightness.general > passed_part);
            }

            if period_start.elapsed() > period_duration {
                period_start = std::time::Instant::now();
            }

            if config.pwm.sleep_nanos != 0 {
                thread::sleep(std::time::Duration::from_nanos(config.pwm.sleep_nanos));
            }

            thread::yield_now();
        }
    });

    sender
}
