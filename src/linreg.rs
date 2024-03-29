use crate::api::get_training_data;
use anyhow::{anyhow, Result};
use std::iter::zip;

pub struct LinRegModel {
    pub data: Vec<i16>,
    pub labels: Vec<i16>,
    slope: Option<f32>,
    intercept: Option<f32>,
}

impl LinRegModel {
    pub fn new() -> LinRegModel {
        LinRegModel {
            data: Vec::new(),
            labels: Vec::new(),
            slope: None,
            intercept: None,
        }
    }

    pub fn populate(&mut self) -> Result<()> {
        // TODO: replace with result type instead of panic. Propogate this error

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

            if data.len() != labels.len() {
                return Err(anyhow!(
                    "Data and labels are not the same length. Data length: {}, labels length: {}",
                    data.len(),
                    labels.len()
                ));
            }

            self.data = data;
            self.labels = labels;
        } else {
            return Err(anyhow!(
                "No training data found! Consider running the application in collection mode."
            ));
        }

        Ok(())
    }

    pub fn train(&mut self) -> Result<()> {
        // TODO: replace with result type instead of panic. Propogate this error

        let x_sum: i16 = self.data.iter().sum();
        let x_mean = x_sum as f32 / self.data.len() as f32;

        let y_sum: i16 = self.labels.iter().sum();
        let y_mean = y_sum as f32 / self.labels.len() as f32;

        let mut weight_1: f32 = 0.0;
        let mut weight_2: f32 = 0.0;
        for (&x, &y) in zip(&self.data, &self.labels) {
            let x = x as f32;
            let y = y as f32;

            weight_1 += (x - x_mean).powi(2);
            weight_2 += (x - x_mean) * (y - y_mean);
        }

        self.slope = Some(weight_2 / weight_1);
        match self.slope {
            Some(slope) => self.intercept = Some(y_mean - slope * x_mean),
            None => return Err(anyhow!("Regression slope is none, for some reason...")),
        }

        Ok(())
    }

    pub fn predict(&self, devices: i16) -> Option<i16> {
        if let (Some(slope), Some(intercept)) = (self.slope, self.intercept) {
            let prediction = Some((slope * devices as f32 + intercept).round() as i16);
            if prediction < Some(0) || prediction > Some(32767) {
                Some(0)  // if prediction is out of bounds, return 0
            } else {
                prediction
            }
        } else {
            None
        }
    }
}
