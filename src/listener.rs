use ansi_term::Colour;
use bluer::Session;
use chrono::Local;
use futures::stream::StreamExt;
use std::collections::HashMap;

use crate::or_continue;

pub async fn listen(threshold: i16, training: bool, bluetooth_adapter_name: &str) {
    let session = Session::new().await.unwrap();
    let adapter = session.adapter(bluetooth_adapter_name).unwrap();

    let mut devices = HashMap::new();

    let mut bluetooth_stream = adapter
        .discover_devices()
        .await
        .expect("Failed creating bluetooth stream. Make sure your bluetooth adapter is turned on");

    while let Some(item) = bluetooth_stream.next().await {
        //println!("Devices connected: {}", devices.len()); // TODO: alternate mode "pinning"

        match item {
            AdapterEvent::DeviceAdded(mac_address) => {
                let device = or_continue!(adapter.device(mac_address));
                let name = or_continue!(device.alias().await);
                let rssi = or_continue!(device.rssi().await);

                if let Some(signalstrength) = rssi {
                    if signalstrength > threshold {
                        devices.insert(mac_address, signalstrength);
                        let output = format!(
                            "[{}] Device added!\t\tMAC: {mac_address}\t\tName: {name}\t\tStrength: {signalstrength} dBm",
                            Local::now()
                        );
                        println!("{}", Colour::Green.paint(&output));
                    }
                }
            }

            AdapterEvent::DeviceRemoved(mac_address) => {
                // Remove all devices that are already registered.
                // Not all devices are registered due to being below the signal strength threshold.
                if devices.remove(&mac_address).is_some() {
                    let output =
                        format!("[{}] Device removed!\t\tMAC: {mac_address}", Local::now());
                    println!("{}", Colour::Red.paint(&output));
                }
            }

            _ => (),
        }
    }
}
