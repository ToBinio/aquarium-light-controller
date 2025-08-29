use std::{
    thread::{self},
    time::Duration,
};

#[derive(Default, Debug)]
struct LightBrightness {
    red: u8,
    blue: u8,
    green: u8,
    general: u8,
}

fn main() {
    let (sender, receiver) = std::sync::mpsc::channel::<LightBrightness>();

    thread::spawn(move || {
        const PERIOD_DURATION: Duration = Duration::from_millis(5);

        let mut brightness = LightBrightness::default();
        let mut period_start = std::time::Instant::now();

        loop {
            if let Ok(new_brightness) = receiver.try_recv() {
                brightness = new_brightness;
            }

            let passed_part: u8 = ((period_start.elapsed().as_nanos() as f32
                / PERIOD_DURATION.as_nanos() as f32)
                * u8::MAX as f32) as u8;

            println!(
                " r{} g{} b{} g{}",
                brightness.red > passed_part,
                brightness.green > passed_part,
                brightness.blue > passed_part,
                brightness.general > passed_part
            );

            if period_start.elapsed() > PERIOD_DURATION {
                period_start = std::time::Instant::now();
            }

            thread::yield_now();
        }
    });

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
            .send(LightBrightness {
                red: brightness,
                blue: brightness,
                green: brightness,
                general: brightness,
            })
            .unwrap();

        thread::sleep(Duration::from_millis(20));
    }
}
