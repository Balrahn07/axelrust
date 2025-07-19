mod config;
mod telemetry;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();
    // Start logging
    telemetry::init();

    let cfg = config::load_config()?;

    tracing::info!("AxelRust is running 🚀 (env: {})", cfg.environment);

    Ok(())
}
