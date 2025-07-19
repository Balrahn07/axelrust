use tracing_subscriber::FmtSubscriber;

/// Initializes the global subscriber for structured logging.
/// Log level is controlled via `LOG_LEVEL` environment variable.
pub fn init() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into()))
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global tracing subscriber");
}