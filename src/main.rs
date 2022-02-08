use ansi_term::Colour;
use chrono::Local;
use structopt::StructOpt;
use tokio::task;
use tokio_cron_scheduler::{Job, JobScheduler};

mod api;
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
    // This is ugly and needs to be refactored
    if opt.training {
        task::spawn(async {
            let mut sched = JobScheduler::new();
            let job = Job::new_async("0 0/5 * * * *", |_uuid, _l| {
                Box::pin(async move {
                    api::insert_training_datapoint().await;
                })
            })
            .unwrap();
            sched.add(job).expect("failed adding job to scheduler");
            sched.start().await.expect("failed starting scheduler");
        });
    } else {
        task::spawn(async {
            let mut sched = JobScheduler::new();
            let job = Job::new_async("0 0/5 * * * *", |_uuid, _l| {
                Box::pin(async move {
                    api::insert_datapoint().await;
                })
            })
            .unwrap();
            sched.add(job).expect("failed adding job to scheduler");
            sched.start().await.expect("failed starting scheduler");
        });
    }

    // Start listening
    listener::listen(opt.threshold, opt.training, opt.adapter.as_str()).await;
}
