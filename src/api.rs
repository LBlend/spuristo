use chrono::{DateTime, Local};
use dotenv_codegen::dotenv;
use serde::{Deserialize, Serialize};

const API_ROOT: &str = dotenv!("SPURISTO_API_ROOT");
const API_TOKEN: &str = dotenv!("SPURISTO_API_TOKEN");

#[derive(Serialize, Deserialize)]
pub struct Datapoint {
    pub time: DateTime<Local>,
    pub devices: i16,
    pub prediction_people: Option<i16>,
    pub actual_people: Option<i16>,
}

pub fn insert_datapoint(devices: i16, prediction_people: Option<i16>) {
    println!("[{}] inserting data into database...", Local::now());

    let datapoint = Datapoint {
        time: Local::now(),
        devices,
        prediction_people,
        actual_people: None,
    };

    insert(datapoint);
}

fn insert(datapoint: Datapoint) {
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(format!("{}/insert", API_ROOT))
        .header("Authorization", format!("Bearer {}", API_TOKEN))
        .json(&datapoint)
        .send();

    match res {
        Ok(res) => {
            if res.status().is_success() {
                println!("Success! - {}", res.status());
            } else {
                println!("Fail! - {}", res.status());
            }
        }
        Err(e) => {
            println!("Failed! - {}", e);
        }
    }
}

pub fn get_training_data() -> Option<Vec<Datapoint>> {
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("{}/training", API_ROOT))
        .header("Authorization", format!("Bearer {}", API_TOKEN))
        .send();

    match res {
        Ok(res) => {
            println!("Success! - {}", res.status());
            let data: Vec<Datapoint> = res
                .json()
                .expect("Failed deserializing JSON training data!");
            Some(data)
        }
        Err(e) => {
            println!("Failed! - {}", e);
            None
        }
    }
}
