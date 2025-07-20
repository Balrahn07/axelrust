pub mod say_hello;

use async_trait::async_trait;
use anyhow::Result;
use std::fmt::Debug;

/// Shared context passed into job execution (for DB, config, etc.)
#[derive(Debug, Clone)]
pub struct JobContext {
    pub job_id: String,
    pub attempt: u32,
}

/// Trait that all jobs must implement.
/// Each job runs asynchronously and returns a Result.
#[async_trait]
pub trait Job: Send + Sync + Debug {
    async fn run(&self, ctx: JobContext) -> Result<()>;
    
    /// Returns the job type (e.g. "send_email")
    fn name(&self) -> &'static str;
}
