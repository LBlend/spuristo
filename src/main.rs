use ansi_term::Colour;
use chrono::Local;
use std::boxed::Box;
use structopt::StructOpt;
use tokio_cron_scheduler::{Job, JobScheduler};

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
    let mut sched = JobScheduler::new();
    if opt.training {
        sched.add(
            Job::new_async("0 */5 * * * *", |uuid, l| {
                Box::pin(async {
                    println!(
                        "[{}] inserting training data into database...",
                        Local::now()
                    );
                    database::insert_training_datapoint().await;
                })
            })
            .unwrap(),
        );
    } else {
        sched.add(
            Job::new_async("0 */5 * * * *", |uuid, l| {
                Box::pin(async {
                    println!("[{}] inserting data into database...", Local::now());
                    database::insert_datapoint().await;
                })
            })
            .unwrap(),
        );
    }
    sched.start().await;

    // Start listening
    listener::listen(opt.threshold, opt.training, opt.adapter.as_str()).await;
}
