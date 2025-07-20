use super::{Job, JobContext};
use async_trait::async_trait;
use anyhow::Result;
use tracing::info;

/// Simple job that prints a hello message with job context.
#[derive(Debug)]
pub struct SayHelloJob {
    pub message: String,
}

#[async_trait]
impl Job for SayHelloJob {
    async fn run(&self, ctx: JobContext) -> Result<()> {
        info!(
            "[{}][attempt:{}] Hello job says: {}",
            ctx.job_id, ctx.attempt, self.message
        );
        Ok(())
    }

    fn name(&self) -> &'static str {
        "say_hello"
    }
}
