use ansi_term::Colour;
use chrono::Local;
use job_scheduler::{Job, JobScheduler};
use std::{thread, time::Duration};
use structopt::StructOpt;

mod api;
use api::database;

mod listener;
pub mod macros;

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

    // Schedule database insertion
    let is_training = opt.training;
    thread::spawn(move || {
        let mut sched = JobScheduler::new();
        let job = if is_training {
            Job::new("0 0/5 * * * *".parse().unwrap(), || {
                println!("[{}] inserting data into database...", Local::now());
                database::insert_training_datapoint();
            })
        } else {
            Job::new("0 0/5 * * * *".parse().unwrap(), || {
                println!("[{}] inserting data into database...", Local::now());
                database::insert_datapoint();
            })
        };

        sched.add(job);

        loop {
            sched.tick();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Start listening
    listener::listen(opt.threshold, opt.training, opt.adapter.as_str()).await;
}
