use chrono::Local;
use reqwest::Client;

pub async fn insert_datapoint() {
    println!("[{}] inserting data into database...", Local::now());
    insert("http://httpbin.org/post").await;
}

pub async fn insert_training_datapoint() {
    println!(
        "[{}] inserting training data into database...",
        Local::now()
    );
    insert("http://httpbin.org/post").await;
}

async fn insert(url: &str) {
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
