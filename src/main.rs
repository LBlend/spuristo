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
                    if signalstrength > threshold {
                        devices.insert(mac_address, signalstrength);
                        let output = &*format!(
                            "[{}] Device added!\t\tMAC: {}\t\tName: {}\t\tStrength: {} dBm",
                            Local::now(),
                            mac_address,
                            name,
                            signalstrength
                        );
                        println!("{}", Colour::Green.paint(output));
                    }
                }
            }

            AdapterEvent::DeviceRemoved(mac_address) => {
                // Remove all devices that are already registered.
                // Not all devices are registered due to being below the signal strength threshold.
                if let Some(_) = devices.remove(&mac_address) {
                    let output =
                        &*format!("[{}] Device removed!\t\tMAC: {}", Local::now(), mac_address);
                    println!("{}", Colour::Red.paint(output));
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
        "Spuristo launched in {} mode with threshold set to {} dBm",
        mode, opt.threshold
    );
    println!("{}", Colour::Blue.bold().paint(launch_message));
    listen(opt.threshold, opt.training).await;
}
