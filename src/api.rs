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

pub async fn insert_datapoint(devices: i16, is_training: bool) {
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

    insert(datapoint).await;
}

async fn insert(datapoint: Datapoint) {
    let client = Client::new();
    let res = client
        .post("http://httpbin.org/post")
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
