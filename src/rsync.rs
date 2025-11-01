//! Таска для синхронизации скинов DDNet
//!
//! Спавним и синхронизируем раз в день, юзаем CONDVAR
//!
//! Папка - .store
//!
//! Нужно фетчить только сам skin и community, только файлы с mime - png, либо просто спарси эту страницу апатча
//!
//! https://ddnet.org/skins/skin/
//! https://ddnet.org/skins/skin/community/
//!
//! лучше хранить lock файл с метой файла

pub mod lock;
pub mod parser;

use futures::future;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::spawn_blocking;
use tracing::info;

use crate::{
    error::Error,
    rsync::{
        lock::Lock,
        parser::{SkinMeta, fetch_skin_list},
    },
};

pub async fn try_sync_skins(lock: Arc<Lock>) -> Result<(), Error> {
    let client = Client::new();

    let urls = vec![
        "https://ddnet.org/skins/skin/",
        "https://ddnet.org/skins/skin/community/",
    ];

    // Параллельно запускаем fetch_skin_list для всех URL-ов
    let fetches = urls.into_iter().map(|url| {
        let client = client.clone();
        async move {
            let list = fetch_skin_list(&client, url).await?;
            info!("{url} -> len PNGs: {:?}", list.len());
            Ok::<_, Error>(list)
        }
    });

    let results: Vec<Vec<SkinMeta>> = future::try_join_all(fetches).await?;

    let map = tokio::task::spawn_blocking(move || {
        use rayon::prelude::*;
        results
            .into_par_iter()
            .flat_map_iter(|v| v.into_iter())
            .fold(
                || HashMap::new(),
                |mut acc: HashMap<String, SkinMeta>, skin: SkinMeta| {
                    acc.entry(skin.name.to_string()).or_insert(skin);
                    acc
                },
            )
            .reduce(
                || HashMap::new(),
                |mut a, b| {
                    for (k, v) in b {
                        a.entry(k).or_insert(v);
                    }
                    a
                },
            )
    })
    .await?;

    let links: Vec<SkinMeta> = map.into_values().collect();

    let lock_for_prepare = lock.clone();
    let to_update = spawn_blocking(move || lock_for_prepare.prepare_to_download(&links)).await?;

    if to_update.is_empty() {
        info!("Nothing to update");
    } else {
        info!("Found something to update: {:#?}", to_update);
        lock.apply_updates(&to_update, &client).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, sync::Arc};

    use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

    use crate::rsync::{lock::Lock, try_sync_skins};

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn fetch() {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("cdn=trace,ohkami=info"));

        let fmt_layer = fmt::layer().compact().with_ansi(true).with_target(true);

        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(env_filter)
            .init();

        unsafe {
            env::set_var("STORE_PATH", "./.store");
        }
        let lock = Arc::new(Lock::read(env::var("STORE_PATH").unwrap()).await.unwrap());
        try_sync_skins(lock).await.unwrap();
    }
}
