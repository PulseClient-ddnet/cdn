//! Кэш, чтобы постоянно не генерить одну и туже тишку. TTL (время жизни) 15 минут
//!
//! Запускаем таску, которая раз в 15 минут коллектит. Юзаем CONDVAR
//!
//! Храним не в опере, а на диске или ты богатый? Ничё потерпят 20 мс, не опухнут.
//!
//! Папка - .cache
//!
//! В коде храним DashMap с <(name,Option<body_color>, Option<feet_color>): Path (absolut)>

use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

use dashmap::DashMap;
use tokio::fs;
use tracing::{info, instrument};

pub const FIVE_MINUTES: Duration = Duration::from_secs(900);

use crate::{app::skin::SkinQuery, error::Error};
#[derive(Debug)]
pub struct CacheItem {
    /// Then it be placed to cache
    pub timestamp: Instant,
    /// It is't relative, it absolute path
    pub path: String,
}

impl CacheItem {
    pub fn new(path_to_skin: String) -> Self {
        Self {
            timestamp: Instant::now(),
            path: path_to_skin,
        }
    }

    #[inline]
    pub fn is_acutal(&self) -> bool {
        self.timestamp.elapsed() <= FIVE_MINUTES
    }
}
#[derive(Debug)]
pub struct CacheStore<'a> {
    pub path: PathBuf,
    pub store: DashMap<SkinQuery<'a>, CacheItem>,
}

impl<'a, 'b: 'a> CacheStore<'a> {
    pub async fn new(path: impl AsRef<Path>) -> Self {
        fs::create_dir(path.as_ref()).await.ok();
        Self {
            path: path.as_ref().to_path_buf(),
            store: DashMap::default(),
        }
    }

    pub async fn save(
        &self,
        query: SkinQuery<'b>,
        data: &[u8],
    ) -> Result<(), Error> {
        let path = self
            .path
            .to_path_buf()
            .join(query.name)
            .with_extension("png");
        fs::write(&path, data).await?;
        self.store.insert(
            query,
            CacheItem::new(path.canonicalize()?.display().to_string()),
        );
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get(
        &self,
        query: SkinQuery<'b>,
    ) -> Result<Option<Vec<u8>>, Error> {
        match self.store.get(&query) {
            Some(x) => {
                if x.value().is_acutal() {
                    info!("Take from cache");
                    Ok(Some(fs::read(&x.value().path).await?))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}
pub type Cache<'a> = Arc<CacheStore<'a>>;
