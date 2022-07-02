use ansi_term::Colour;
use bluer::Address;
use dotenv::dotenv;
use job_scheduler::{Job, JobScheduler};
use linreg::LinRegModel;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use structopt::StructOpt;

#[macro_use]
extern crate dotenv_codegen;

mod api;
mod linreg;
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

    // Create empty classifier. Fetch data and train it if we're not in collection mode
    let mut model = LinRegModel::new();
    if !is_collecting {
        model.populate();

        // Simulate populating data by manually doing it here
        /* let data: Vec<i16> = vec![2, 8, 6, 4, 9, 3, 12];
        classifier.data = data;

        let labels = vec![0, 3, 3, 2, 4, 1, 9];
        classifier.labels = labels; */

        model.train();
    }

    // Schedule database insertion every 5 minutes
    std::thread::spawn(move || {
        let mut sched = JobScheduler::new();

        let job = Job::new("0 0/5 * * * *".parse().unwrap(), || {
            let devices = device_map.lock().unwrap().len() as i16;

            // This is really fucking dumb
            if !is_collecting {
                api::insert_datapoint(devices, model.predict(devices));
            } else {
                api::insert_datapoint(devices, None);
            }
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
