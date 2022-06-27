use crate::api::get_training_data;

pub struct Classifier {
    data: Vec<i16>,
    labels: Vec<i16>,
}

impl Classifier {
    pub fn new() -> Classifier {
        Classifier {
            data: Vec::new(),
            labels: Vec::new(),
        }
    }

    pub fn populate(&mut self) {
        let training_data = get_training_data();

        if let Some(training_data) = training_data {
            let data: Vec<i16> = training_data
                .iter()
                .map(|datapoint| datapoint.devices)
                .collect();
            let labels: Vec<i16> = training_data
                .iter()
                .filter_map(|datapoint| datapoint.actual_people)
                .collect();

            if data.len() == labels.len() {
                panic!("Data and labels are not the same length");
            }

            self.data = data;
            self.labels = labels;
        }
        panic!("No training data found!");
    }

    pub fn train(&mut self) {
        todo!();
    }

    pub fn predict(&self, devices: i16) -> i16 {
        todo!();
    }
}
