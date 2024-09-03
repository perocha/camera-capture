// src/main.rs

mod camera;
mod mqtt;
mod image_processing;
mod config;

use log::{info, error};
use std::time::Duration;
use mqtt::setup_mqtt;
use camera::setup_camera;
use camera::capture_frame;
use image_processing::encode_frame;
use mqtt::publish_frame;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Set up MQTT client
    let mqttoptions = setup_mqtt();
    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    // Set up the camera
    let mut cam = setup_camera()?;

    // Main capture loop
    loop {
        // Capture and process the frame
        match capture_frame(&mut cam) {
            Ok(frame) => {
                // Encode frame as JPEG
                match encode_frame(&frame) {
                    Ok(encoded_frame) => {
                        // Publish frame over MQTT
                        publish_frame(&mut client, &encoded_frame);
                    }
                    Err(e) => error!("Failed to encode frame: {}", e),
                }
            }
            Err(e) => error!("Failed to capture frame: {}", e),
        }

        // Handle MQTT connection (optional for debugging purposes)
        for notification in connection.iter() {
            info!("MQTT notification: {:?}", notification);
        }

        // Sleep or throttle loop if needed
        std::thread::sleep(Duration::from_millis(100));
    }
}
