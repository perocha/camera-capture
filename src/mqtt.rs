// src/mqtt.rs

use rumqttc::{MqttOptions, Client, QoS};
use log::{info, error};
use std::time::Duration;
use crate::config::{MQTT_BROKER, MQTT_PORT, MQTT_TOPIC};

// Set up the MQTT client
pub fn setup_mqtt() -> MqttOptions {
    let mut mqttoptions = MqttOptions::new("rust_client", MQTT_BROKER, MQTT_PORT);
    mqttoptions.set_keep_alive(Duration::from_secs(60));
    mqttoptions
}

// Publish the encoded frame over MQTT
pub fn publish_frame(client: &mut Client, encoded_frame: &str) {
    match client.publish(MQTT_TOPIC, QoS::AtLeastOnce, false, encoded_frame) {
        Ok(_) => info!("Frame published to MQTT."),
        Err(e) => error!("Failed to publish frame: {}", e),
    }
}
