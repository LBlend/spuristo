use ansi_term::Colour;
use bluer::Address;
use classifier::Classifier;
use dotenv::dotenv;
use job_scheduler::{Job, JobScheduler};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use structopt::StructOpt;

#[macro_use]
extern crate dotenv_codegen;

mod api;
mod classifier;
mod listener;
mod macros;

// CLI arguments struct
#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    collection: bool,

    #[structopt(default_value = "-75", long)]
    threshold: i16,

    #[structopt(default_value = "hci0", long)]
    adapter: String,
}

#[tokio::main]
pub async fn main() {
    // Load environment variables
    dotenv().ok();

    // Load CLI arguments
    let opt = Opt::from_args();

    // Start message
    let mode = if opt.collection {
        "collection"
    } else {
        "prediction"
    };
    let launch_message = format!(
        "Spuristo launched in {mode} mode with threshold set to {} dBm on adapter {}",
        opt.threshold, opt.adapter
    );
    println!("{}", Colour::Blue.bold().paint(&launch_message));

    // Create shared state keeping track of devices
    let device_map: Arc<Mutex<HashMap<Address, i16>>> = Arc::new(Mutex::new(HashMap::new()));
    let device_map_listen = Arc::clone(&device_map);

    let is_collecting = opt.collection;
    let is_collecting = true; // Temporary

    // Create empty classifier. Fetch data and train it if we're not in collection mode
    let mut classifier = Classifier::new();
    if !is_collecting {
        classifier.populate();
        classifier.train();
    }

    // Schedule database insertion every 5 minutes
    std::thread::spawn(move || {
        let mut sched = JobScheduler::new();

        let job = Job::new("0 0/5 * * * *".parse().unwrap(), || {
            let devices = device_map.lock().unwrap().len() as i16;
            let prediction_people = if is_collecting {
                None
            } else {
                Some(classifier.predict(devices))
            };
            api::insert_datapoint(devices, prediction_people);
        });

        sched.add(job);
        loop {
            sched.tick();
            std::thread::sleep(Duration::from_millis(500));
        }
    });

    // Start listening
    listener::listen(device_map_listen, opt.threshold, opt.adapter.as_str()).await;
}
