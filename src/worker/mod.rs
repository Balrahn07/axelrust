use crate::jobs::{Job, JobContext};
use anyhow::Result;
use tracing::{error, info};

/// In-memory job runner that executes each job in sequence.
pub struct JobRunner;

impl JobRunner {
    /// Runs a list of jobs with their associated contexts.
    pub async fn run_jobs(jobs: Vec<(Box<dyn Job>, JobContext)>) -> Result<()> {
        for (job, ctx) in jobs {
            info!("Running job '{}'", job.name());

            let result = job.run(ctx.clone()).await;

            match result {
                Ok(_) => info!("Job '{}' completed successfully", job.name()),
                Err(e) => error!("Job '{}' failed: {:?}", job.name(), e),
            }
        }

        Ok(())
    }
}
