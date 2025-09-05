use futures::stream::StreamExt;
use serde::Deserialize;

use reqwest_eventsource::{Event, EventSource};

use crate::{config::Config, pwm::LightBrightness};

pub mod config;
pub mod pwm;

#[derive(Deserialize)]
struct LightUpdateRequest {
    color: String,
    brightness: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::open().unwrap();

    let sender = pwm::spawn(&config);

    let mut event_source = EventSource::get(format!("{}/api/see", &config.base_url));
    while let Some(event) = event_source.next().await {
        match event {
            Ok(Event::Open) => println!("Connection Open!"),
            Ok(Event::Message(message)) => {
                let request: LightUpdateRequest = serde_json::from_str(&message.data)?;

                let color = request.color; //hex string
                let brightness = request.brightness;

                let red = u8::from_str_radix(&color[1..3], 16).unwrap() as f32 * brightness;
                let green = u8::from_str_radix(&color[3..5], 16).unwrap() as f32 * brightness;
                let blue = u8::from_str_radix(&color[5..7], 16).unwrap() as f32 * brightness;

                sender
                    .send(LightBrightness::new(
                        red as u8,
                        green as u8,
                        blue as u8,
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
