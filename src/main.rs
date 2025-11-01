use tracing::{Level, info};

use crate::app::app;

pub mod app;
pub mod cache;
pub mod error;
pub mod rsync;

fn init_logger(level: Level) {
    use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};

    // Check for RUST_LOG env var, default to INFO if not present
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("cdn=debug,ohkami=info"));

    let fmt_layer = fmt::layer().compact().with_ansi(true).with_target(true);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env_filter)
        .init();

    info!("Logger initialized at level: {}", level);
}

#[tokio::main]
async fn main() {
    init_logger(Level::INFO);
    app().await
}
