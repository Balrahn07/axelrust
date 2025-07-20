mod config;
mod telemetry;
mod jobs;

use anyhow::Result;
use jobs::{say_hello::SayHelloJob, Job, JobContext};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let cfg = config::load_config()?;
    // Start logging
    telemetry::init_with_level(&cfg.log_level);

    tracing::info!("AxelRust is running 🚀 (env: {})", cfg.environment);

    // Simulate a job execution
    let job = SayHelloJob {
        message: "This is the first job in AxelRust!".into(),
    };

    let ctx = JobContext {
        job_id: "job-001".into(),
        attempt: 1,
    };

    job.run(ctx).await?;

    Ok(())
}
