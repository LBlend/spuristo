use chrono::Local;
use reqwest::Client;
use crate::classifier;

pub async fn insert_datapoint(devices: i16) {
    println!("[{}] inserting data into database...", Local::now());
    let prediction = classifier::predict(devices);
    insert("http://httpbin.org/post", devices, Some(prediction)).await;
}

pub async fn insert_training_datapoint(devices: i16) {
    println!(
        "[{}] inserting training data into database...",
        Local::now()
    );
    insert("http://httpbin.org/post", devices, None).await;
}

async fn insert(url: &str, devices: i16, prediction: Option<i16>) {
    println!("Received {devices} device(s)");
    let client = Client::new();
    let res = client.post(url).send().await;
    match res {
        Ok(res) => {
            println!("Success! - {}", res.status());
        }
        Err(e) => {
            println!("Failed! - {}", e);
        }
    }
}
