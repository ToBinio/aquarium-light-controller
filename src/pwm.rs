use std::{thread, time::Duration};

use rppal::gpio::{Gpio, OutputPin};

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

    fn calc_pulse_width(config: &Config, brightness: u8) -> Duration {
        Duration::from_nanos(
            (config.pwm.period_nanos as f64 * brightness as f64 / u8::MAX as f64) as u64,
        )
    }
}

pub fn spawn(config: &Config) -> std::sync::mpsc::Sender<LightBrightness> {
    let (sender, receiver) = std::sync::mpsc::channel::<LightBrightness>();

    let config = config.clone();
    thread::spawn(move || {
        let period_duration: Duration = Duration::from_nanos(config.pwm.period_nanos);

        let gpio = Gpio::new().ok();
        let mut pins = Pins::from_config(&config, &gpio);

        loop {
            let brightness = receiver.recv().unwrap();

            if let Some(pins) = &mut pins {
                pins.red
                    .set_pwm(
                        period_duration,
                        Pins::calc_pulse_width(&config, brightness.red),
                    )
                    .unwrap();
                pins.green
                    .set_pwm(
                        period_duration,
                        Pins::calc_pulse_width(&config, brightness.green),
                    )
                    .unwrap();
                pins.blue
                    .set_pwm(
                        period_duration,
                        Pins::calc_pulse_width(&config, brightness.blue),
                    )
                    .unwrap();
                pins.general
                    .set_pwm(
                        period_duration,
                        Pins::calc_pulse_width(&config, brightness.general),
                    )
                    .unwrap();
            } else {
                println!("{:?}", brightness);
            }
        }
    });

    sender
}
