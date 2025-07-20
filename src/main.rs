mod config;
mod telemetry;
mod worker;
mod jobs;

use anyhow::Result;
use jobs::{say_hello::SayHelloJob, Job, JobContext};
use worker::JobRunner;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let cfg = config::load_config()?;
    // Start logging
    telemetry::init_with_level(&cfg.log_level);

    tracing::info!("AxelRust is running 🚀 (env: {})", cfg.environment);

    let job1 = SayHelloJob {
        message: "This is the first job in AxelRust!".into(),
    };

    let job2 = SayHelloJob {
        message: "Another task for the queue!".into(),
    };

    let jobs = vec![
        (Box::new(job1) as Box<dyn Job>, JobContext {
            job_id: "job-001".into(),
            attempt: 1,
        }),
        (Box::new(job2) as Box<dyn Job>, JobContext {
            job_id: "job-002".into(),
            attempt: 1,
        }),
    ];

    JobRunner::run_jobs(jobs).await?;

    Ok(())
}