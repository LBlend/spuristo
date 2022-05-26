use ansi_term::Colour;
use bluer::Address;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use tokio::task;
use tokio_cron_scheduler::{Job, JobScheduler};

mod api;
mod classifier;
mod listener;
mod macros;

// CLI arguments struct
#[derive(StructOpt)]
struct Opt {
    #[structopt(default_value = "-75", long)]
    threshold: i16,

    #[structopt(short, long)]
    training: bool,

    #[structopt(default_value = "hci0", long)]
    adapter: String,
}

#[tokio::main]
pub async fn main() {
    // Load environment variables
    dotenv().ok();
    let api_root = env::var("SPURISTO_API_ROOT").expect("$SPURISTO_API_ROOT is not set");
    let api_token = env::var("SPURISTO_API_TOKEN").expect("$SPURISTO_API_TOKEN is not set");

    let opt = Opt::from_args();

    // Start message
    let mode = if opt.training {
        "training"
    } else {
        "prediction"
    };
    let launch_message = format!(
        "Spuristo launched in {mode} mode with threshold set to {} dBm on adapter {}",
        opt.threshold, opt.adapter
    );
    println!("{}", Colour::Blue.bold().paint(&launch_message));

    let device_map: Arc<Mutex<HashMap<Address, i16>>> = Arc::new(Mutex::new(HashMap::new()));
    let device_map_listen = Arc::clone(&device_map);

    // Schedule database insertion every 5 minutes
    let is_training = opt.training.clone(); // Ew, but necessary
    task::spawn(async move {
        let mut sched = JobScheduler::new();
        let job = Job::new_async("0 0/5 * * * *", move |_uuid, _l| {
            let device_map_cron = Arc::clone(&device_map);
            let api_root = api_root.clone();
            let api_token = api_token.clone();
            Box::pin(async move {
                let devices = device_map_cron.lock().unwrap().len();
                api::insert_datapoint(devices as i16, is_training, &api_root, &api_token).await;
            })
        })
        .unwrap();
        sched.add(job).expect("failed adding job to scheduler");
        sched.start().await.expect("failed starting scheduler");
    });

    // Start listening
    listener::listen(
        device_map_listen,
        opt.threshold,
        opt.adapter.as_str(),
    )
    .await;
}
