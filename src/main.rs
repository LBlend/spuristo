use ansi_term::Colour;
use bluer::*;
use futures::stream::StreamExt;
use std::collections::HashMap;

// Global constants
const THRESHOLD: i16 = -75; // Blutetooth strength in dBm
const BLUETOOTH_ADAPTER_NAME: &str = "hci0";

pub async fn listen() {
    let session = Session::new().await.unwrap();
    let adapter = session.adapter(BLUETOOTH_ADAPTER_NAME).unwrap();

    let mut devices = HashMap::new();

    let mut bluetooth_stream = adapter
        .discover_devices()
        .await
        .expect("Failed creating bluetooth stream. Make sure your bluetooth adapter is turned on");

    while let Some(item) = bluetooth_stream.next().await {
        println!("{} Devices connected", devices.len()); // TODO: alternate mode "pinning"

        match item {
            AdapterEvent::DeviceAdded(mac_address) => {
                let device = adapter.device(mac_address).unwrap();
                let name = device
                    .alias()
                    .await
                    .expect("Alias of added device couldn't be found");
                let rssi = device
                    .rssi()
                    .await
                    .expect("Signal strength couldn't be found");

                if let Some(signalstrength) = rssi {
                    if signalstrength > THRESHOLD {
                        devices.insert(mac_address, signalstrength);
                        let output = &*format!(
                            "Device added!\t\tMAC: {}\t\tName: {}\t\tStrength: {} dBm",
                            mac_address, name, signalstrength
                        );
                        println!("{}", Colour::Green.paint(output));
                    }
                }
            }

            AdapterEvent::DeviceRemoved(mac_address) => {
                devices.remove(&mac_address);
                let output = &*format!("Device removed!\t\tMAC: {}", mac_address);
                println!("{}", Colour::Red.paint(output));
            }

            _ => (),
        }
    }
}

#[tokio::main]
pub async fn main() {
    listen().await;
}
