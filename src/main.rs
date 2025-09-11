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
    SetColor { hex: String },
    SetBrightness { value: f32 },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::open().unwrap();

    let sender = pwm::spawn(&config);

    let mut color = "#FFFFFF".to_string();
    let mut brightness = 0.;
    let mut active = true;

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
                    UpdateBody::SetColor { hex } => {
                        color = hex;
                    }
                    UpdateBody::SetBrightness { value } => {
                        brightness = value;
                    }
                }

                if !active {
                    sender.send(LightBrightness::new(0, 0, 0, 0)).unwrap();
                    continue;
                }

                let red = u8::from_str_radix(&color[1..3], 16).unwrap() as f32 * 0.5;
                let green = u8::from_str_radix(&color[3..5], 16).unwrap() as f32 * 0.5;
                let blue = u8::from_str_radix(&color[5..7], 16).unwrap() as f32 * 0.5;

                sender
                    .send(LightBrightness::new(
                        red as u8 + (brightness * 128.) as u8,
                        green as u8 + (brightness * 128.) as u8,
                        blue as u8 + (brightness * 128.) as u8,
                        (brightness * 255.) as u8,
                    ))
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
