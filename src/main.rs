mod config;
mod telemetry;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();
    let cfg = config::load_config()?;
    // Start logging
    telemetry::init_with_level(&cfg.log_level);

    tracing::info!("AxelRust is running 🚀 (env: {})", cfg.environment);

    Ok(())
}
