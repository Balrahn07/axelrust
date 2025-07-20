mod config;
mod jobs;
mod telemetry;
mod worker;

use anyhow::Result;
use jobs::JobContext;
use jobs::registry::JobDispatcher;
use worker::JobRunner;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let cfg = config::load_config()?;
    telemetry::init_with_level(&cfg.log_level);

    tracing::info!("AxelRust is running 🚀 (env: {})", cfg.environment);

    // Simulate dispatching jobs by type and payload
    let job1 = JobDispatcher::dispatch("say_hello", "This is the first job in AxelRust!")?;
    let job2 = JobDispatcher::dispatch("say_hello", "Another task for the queue!")?;

    let jobs = vec![
        (
            job1,
            JobContext {
                job_id: "job-001".into(),
                attempt: 1,
            },
        ),
        (
            job2,
            JobContext {
                job_id: "job-002".into(),
                attempt: 1,
            },
        ),
    ];

    JobRunner::run_jobs(jobs).await?;

    Ok(())
}
