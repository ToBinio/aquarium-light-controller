use futures::stream::StreamExt;
use serde::Deserialize;

use reqwest_eventsource::{Event, EventSource};

use crate::{config::Config, pwm::LightBrightness};

pub mod config;
pub mod pwm;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum UpdateBody {
    Activate { on: bool },
    SetColor { hue: f32, saturation: f32 },
    SetBrightness { brightness: f32 },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::open().unwrap();

    let sender = pwm::spawn(&config);

    let mut hue = 0.;
    let mut saturation = 0.;
    let mut brightness = 0.;

    let mut active = false;

    let mut event_source = EventSource::get(format!("{}/api/see", &config.base_url));
    while let Some(event) = event_source.next().await {
        match event {
            Ok(Event::Open) => println!("Connection Open!"),
            Ok(Event::Message(message)) => {
                let request: UpdateBody = serde_json::from_str(&message.data)?;

                match request {
                    UpdateBody::Activate { on } => {
                        active = on;
                    }
                    UpdateBody::SetColor {
                        hue: hue_value,
                        saturation: saturation_value,
                    } => {
                        hue = hue_value;
                        saturation = saturation_value;
                    }
                    UpdateBody::SetBrightness { brightness: value } => {
                        brightness = value;
                    }
                }

                if !active {
                    sender.send(LightBrightness::new(0, 0, 0, 0)).unwrap();
                    continue;
                }
                sender
                    .send(hsb_to_light_brightness(hue, saturation, brightness))
                    .unwrap();
            }
            Err(err) => {
                println!("Error: {}", err);
                event_source.close();
            }
        }
    }

    Ok(())
}

fn hsb_to_light_brightness(hue: f32, saturation: f32, brightness: f32) -> LightBrightness {
    let c = brightness * saturation;
    let x = c * (1.0 - (((hue * 6.0) % 2.0) - 1.0).abs());
    let m = brightness - c;

    let mut r = 0.;
    let mut g = 0.;
    let mut b = 0.;

    if hue >= 0. && hue < (1. / 6.) {
        r = c;
        g = x;
        b = 0.;
    } else if hue >= (1. / 6.) && hue < (2. / 6.) {
        r = x;
        g = c;
        b = 0.;
    } else if hue >= (2. / 6.) && hue < (3. / 6.) {
        r = 0.;
        g = c;
        b = x;
    } else if hue >= (3. / 6.) && hue < (4. / 6.) {
        r = 0.;
        g = x;
        b = c;
    } else if hue >= (4. / 6.) && hue < (5. / 6.) {
        r = x;
        g = 0.;
        b = c;
    } else if hue >= (5. / 6.) && hue <= 1. {
        r = c;
        g = 0.;
        b = x;
    }

    let red = ((r + m) * 255.) as u8;
    let green = ((g + m) * 255.) as u8;
    let blue = ((b + m) * 255.) as u8;

    LightBrightness::new(red, green, blue, (brightness * 255.) as u8)
}
