// src/image_processing.rs

use opencv::prelude::*;
use opencv::imgcodecs::imencode;
use opencv::core::Mat;
use base64;
use log::error;

// Encode the frame as JPEG and then to base64
pub fn encode_frame(frame: &Mat) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = opencv::core::Vector::<u8>::new();
    imencode(".jpg", &frame, &mut buffer, &opencv::core::Vector::new())?;
    let encoded_frame = base64::encode(&buffer);
    Ok(encoded_frame)
}
