use chrono::Local;
use reqwest::Client;

pub async fn insert_datapoint(devices: i16) {
    println!("[{}] inserting data into database...", Local::now());
    insert("http://httpbin.org/post", devices).await;
}

pub async fn insert_training_datapoint(devices: i16) {
    println!(
        "[{}] inserting training data into database...",
        Local::now()
    );
    insert("http://httpbin.org/post", devices).await;
}

async fn insert(url: &str, devices: i16) {
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
