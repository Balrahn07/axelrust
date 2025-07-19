use tracing_subscriber::{FmtSubscriber, EnvFilter};

/// Initializes the global subscriber for structured logging.
/// Log level is controlled via `LOG_LEVEL` environment variable.
pub fn init_with_level(level: &str) {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new(level))
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global tracing subscriber");
}