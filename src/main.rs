use std::{sync::Arc, time::Duration};

use tokio::{fs, time::sleep};
use tracing::{Level, info};

use crate::{
    app::app,
    cache::CacheStore,
    rsync::{lock::LockStore, try_sync_skins},
};

pub mod app;
pub mod cache;
pub mod error;
pub mod rsync;

fn init_logger(level: Level) {
    use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};

    // Check for RUST_LOG env var, default to INFO if not present
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("cdn=trace,ohkami=trace"));

    let fmt_layer = fmt::layer().compact().with_ansi(true).with_target(true);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env_filter)
        .init();

    info!("Logger initialized at level: {}", level);
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    init_logger(Level::INFO);
    let address = format!(
        "0.0.0.0:{}",
        std::env::var("PORT").expect("PORT must be set")
    );
    let lock = Arc::new(
        LockStore::read(std::env::var("STORE_PATH").expect("STORE_PATH must be set"))
            .await
            .unwrap(),
    );
    let cache = Arc::new(CacheStore::new().await);

    fs::create_dir("static").await.ok();

    '_SYNC: {
        try_sync_skins(lock.clone()).await.expect("why");
        let lock = lock.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(60 * 60)).await;

                if let Err(err) = try_sync_skins(lock.clone()).await {
                    tracing::error!("Ошибка при выполнении try_sync_skins: {err:?}");
                } else {
                    tracing::info!("Скины успешно синхронизированы");
                }
            }
        });
    }

    '_CACHE: {
        let cache = cache.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(60 * 60)).await;
                let keys: Vec<_> = cache
                    .store
                    .iter()
                    .map(|entry| entry.key().clone())
                    .collect();

                for key in keys {
                    info!(name=%key.name, body=?key.body, feet=?key.feet, "Item has removed from cache");
                    cache.store.remove(&key);
                }
                tracing::info!("Cache cleared");
            }
        });
    }

    app(lock, cache, &address).await
}
