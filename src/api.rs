use crate::classifier;
use chrono::{DateTime, Local};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Datapoint {
    time: DateTime<Local>,
    devices: i16,
    prediction_people: Option<i16>,
    actual_people: Option<i16>,
}

pub async fn insert_datapoint(devices: i16, is_training: bool, api_root: &str, api_token: &str) {
    println!("[{}] inserting data into database...", Local::now());

    let mut datapoint = Datapoint {
        time: Local::now(),
        devices,
        prediction_people: None,
        actual_people: None,
    };

    if !is_training {
        let prediction_people = classifier::predict(devices);
        datapoint.prediction_people = Some(prediction_people);
    }

    insert(datapoint, api_root, api_token).await;
}

async fn insert(datapoint: Datapoint, api_root: &str, api_token: &str) {
    let client = Client::new();
    let res = client
        .post("{api_root}/insert")
        .header("Authorization", format!("Bearer {}", api_token))
        .json(&datapoint)
        .send()
        .await;
    match res {
        Ok(res) => {
            println!("Success! - {}", res.status());
        }
        Err(e) => {
            println!("Failed! - {}", e);
        }
    }
}
