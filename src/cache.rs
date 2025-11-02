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
    sync::Arc,
    time::{Duration, Instant},
};

use bytes::Bytes;
use dashmap::DashMap;
use tracing::info;

pub const FIVE_MINUTES: Duration = Duration::from_secs(900);

use crate::{app::skin::SkinQuery, error::Error};
#[derive(Debug)]
pub struct CacheItem {
    /// Then it be placed to cache
    pub timestamp: Instant,
    /// `Tee`'s data
    pub data: Bytes,
}

impl CacheItem {
    pub fn new(data: Bytes) -> Self {
        Self {
            timestamp: Instant::now(),
            data,
        }
    }

    #[inline]
    pub fn is_acutal(&self) -> bool {
        self.timestamp.elapsed() <= FIVE_MINUTES
    }
}
#[derive(Debug)]
pub struct CacheStore {
    pub store: DashMap<SkinQuery, CacheItem>,
}

impl CacheStore {
    pub async fn new() -> Self {
        Self {
            store: DashMap::default(),
        }
    }

    pub async fn save(
        &self,
        query: SkinQuery,
        data: Bytes,
    ) -> Result<(), Error> {
        self.store.insert(query, CacheItem::new(data));
        Ok(())
    }

    pub async fn get(
        &self,
        query: &SkinQuery,
    ) -> Result<Option<Bytes>, Error> {
        match self.store.get(&query) {
            Some(x) => {
                if x.value().is_acutal() {
                    info!("Take from cache");
                    Ok(Some(x.value().data.clone()))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}
pub type Cache = Arc<CacheStore>;
