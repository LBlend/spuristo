use bluer::*;
use futures::stream::{StreamExt};
use std::collections::HashMap;


const THRESHOLD: i16 = -75;  // Blutetooth strength in dBm
const BLUETOOTH_ADAPTER_NAME: &str = "hci0";


pub async fn yeet() {
    let session = Session::new().await.expect("I hate error handling");
    let adapter = session.adapter(BLUETOOTH_ADAPTER_NAME).expect("I hate error handling");

    let mut devices = HashMap::new();

    let mut bluetooth_stream = adapter.discover_devices().await.expect("a I really hate error handling");
    while let Some(item) = bluetooth_stream.next().await {
        match item {
            AdapterEvent::DeviceAdded(mac_address) => {
                let device = adapter.device(mac_address).expect("b I really hate error handling");
                let name = device.alias().await.expect("A");
                let rssi = device.rssi().await.expect("yeet");
                match rssi {
                    Some(signalstrength) => {
                        if signalstrength > THRESHOLD {
                            devices.insert(mac_address, signalstrength);
                            println!("MAC: {}\tName: {}\tStrength: {}", mac_address, name, signalstrength);
                            println!("{:?} Devices connected", devices.len());
                        }
                    },
                    None => ()
                }
                ()
            },
            AdapterEvent::DeviceRemoved(mac_address) => {
                devices.remove(&mac_address);
                ()
            },
            AdapterEvent::PropertyChanged(property) => {
                match property {
                    AdapterProperty::Address(address) => println!("address changed to:\t{}", address),
                    _ => ()
                }
            }
            _ => ()
        }
    }
}

#[tokio::main]
pub async fn main() {
    yeet().await;
    println!("Jeg hater Rust");
}