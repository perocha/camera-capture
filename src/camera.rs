// src/camera.rs

use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};
use opencv::core::Mat;
use log::{info, error};

// Set up the camera
pub fn setup_camera() -> Result<VideoCapture, opencv::Error> {
    let mut cam = VideoCapture::new(0, CAP_ANY)?; // Open the default camera
    if !cam.is_opened()? {
        error!("Error: Unable to open camera.");
        return Err(opencv::Error::new(0, String::from("Cannot open camera.")));
    }
    info!("Camera successfully opened.");
    Ok(cam)
}

// Capture a frame from the camera
pub fn capture_frame(cam: &mut VideoCapture) -> Result<Mat, opencv::Error> {
    let mut frame = Mat::default();
    cam.read(&mut frame)?;
    if frame.empty() {
        error!("Error: Captured empty frame.");
        return Err(opencv::Error::new(0, String::from("Captured empty frame.")));
    }
    info!("Frame captured successfully.");
    Ok(frame)
}
