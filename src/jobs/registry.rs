use super::{Job, say_hello::SayHelloJob};
use anyhow::{Result, bail};

/// Dispatcher responsible for mapping job names to executable Job structs.
pub struct JobDispatcher;

impl JobDispatcher {
    /// Given a job name and payload, returns a boxed Job instance.
    pub fn dispatch(name: &str, payload: &str) -> Result<Box<dyn Job>> {
        match name {
            "say_hello" => Ok(Box::new(SayHelloJob {
                message: payload.to_string(),
            })),
            _ => bail!("No job registered with name '{}'", name),
        }
    }
}