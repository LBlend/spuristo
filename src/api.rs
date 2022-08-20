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
    let req = ureq::post(format!("{}/insert", API_ROOT).as_str())
        .set("Authorization", format!("Bearer {}", API_TOKEN).as_str())
        .send_json(&datapoint);

    match req {
        Ok(res) => {
            if res.status() >= 200 && res.status() < 300 {  // Yeah this is kind of a shit way to do this
                println!("Success! - {}", res.status());
            } else {
                println!("Error! - {}", res.status());
            }
        }
        Err(e) => {
            println!("Failed! - {}", e);
        }
    }
}

pub fn get_training_data() -> Option<Vec<Datapoint>> {
    let req = ureq::get(format!("{}/training", API_ROOT).as_str())
        .set("Authorization", format!("Bearer {}", API_TOKEN).as_str())
        .call();
    
    match req {
        Ok(res) => {
            println!("Successfully retrieved training data!");
            let data = res
                .into_json()
                .expect("Failed deserializing JSON training data!");
            Some(data)
        }
        Err(e) => {
            println!("Failed retrieving training data! - {}", e);
            None
        }
    }
}
