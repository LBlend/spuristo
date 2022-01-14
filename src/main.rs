use ansi_term::Colour;
use bluer::*;
use chrono::Local;
use futures::stream::StreamExt;
use std::collections::HashMap;
use structopt::StructOpt;

// Global constants
const BLUETOOTH_ADAPTER_NAME: &str = "hci0";

// CLI arguments struct
#[derive(StructOpt)]
struct Opt {
    #[structopt(default_value = "-75", long)]
    threshold: i16,

    #[structopt(short, long)]
    training: bool,
}

pub async fn listen(threshold: i16, training: bool) {
    let session = Session::new().await.unwrap();
    let adapter = session.adapter(BLUETOOTH_ADAPTER_NAME).unwrap();

    let mut devices = HashMap::new();

    let mut bluetooth_stream = adapter
        .discover_devices()
        .await
        .expect("Failed creating bluetooth stream. Make sure your bluetooth adapter is turned on");

    while let Some(item) = bluetooth_stream.next().await {
        println!("Devices connected: {}", devices.len()); // TODO: alternate mode "pinning"

        match item {
            AdapterEvent::DeviceAdded(mac_address) => {
                let device = match adapter.device(mac_address) {
                    Ok(device) => device,
                    Err(_) => continue,
                };
                let name = match device.alias().await {
                    Ok(name) => name,
                    Err(_) => continue,
                };
                let rssi = match device.rssi().await {
                    Ok(rssi) => rssi,
                    Err(_) => continue,
                };

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

#[tokio::main]
pub async fn main() {
    let opt = Opt::from_args();
    let mode = if opt.training {
        "training"
    } else {
        "prediction"
    };
    let launch_message = format!(
        "Spuristo launched in {mode} mode with threshold set to {} dBm",
        opt.threshold
    );
    println!("{}", Colour::Blue.bold().paint(&launch_message));
    listen(opt.threshold, opt.training).await;
}
