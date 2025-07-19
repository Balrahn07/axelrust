mod config;
mod telemetry;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    telemetry::init();

    let cfg = config::load_config()?;

    tracing::info!("AxelRust is running 🚀 (env: {})", cfg.environment);

    Ok(())
}
