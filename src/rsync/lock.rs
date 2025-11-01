use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use dashmap::DashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::{
    fs,
    io::{self, AsyncWriteExt},
    sync::Semaphore,
    task::JoinSet,
};
use tracing::{error, info, warn};

use crate::{error::Error, rsync::parser::SkinMeta};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LockMeta {
    /// saved os path
    pub path: String,
    /// Download link
    pub origin: String,
    /// Last update timestamp
    pub ita: String,
}

#[derive(Debug)]
pub struct Lock {
    /// Path to store
    pub path: PathBuf,
    pub inner: Arc<DashMap<String, LockMeta>>,
}

impl Lock {
    /// Save inner
    pub async fn save(&self) -> io::Result<()> {
        let file = serde_json::to_string_pretty(&*self.inner).unwrap();
        fs::write(&self.path.join("lock.json"), file).await?;
        Ok(())
    }

    pub async fn read(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();
        let lock_path = path.join("lock.json");

        fs::create_dir(path).await.ok();

        // Если файл непустой — читаем содержимое
        if lock_path.exists() {
            let content = fs::read_to_string(&lock_path).await?;
            let inner = serde_json::from_str(&content)?;

            Ok(Self {
                path: path.to_path_buf(),
                inner: Arc::new(inner),
            })
        } else {
            let inner = Arc::new(DashMap::new());
            fs::write(&lock_path, serde_json::to_string_pretty(&*inner).unwrap()).await?;
            Ok(Self {
                path: path.to_path_buf(),
                inner,
            })
        }
    }

    /// Сравнивает lock с skins и возвращает список новых или устаревших скинов.
    pub fn prepare_to_download(
        &self,
        skins: &[SkinMeta],
    ) -> Vec<SkinMeta> {
        // Собираем список всех скинов, которых нет или они устарели
        skins
            .par_iter()
            .filter_map(|skin| {
                match self.inner.get(&skin.name) {
                    Some(lock_meta) if !skin.eq_lock_meta(lock_meta.value()) => {warn!(name=%skin.name, meta_ita=%lock_meta.value().ita, current_ita=%skin.ita, "↗️ Found outdated skin"); Some(skin.clone())}, // Устаревший
                    None => {info!(name=%skin.name, "↖️ Found new skin"); Some(skin.clone())}, // Новый скин
                    _ => None,                  // Совпадает по ita — не трогаем
                }
            })
            .collect()
    }

    /// После скачивания можно обновить lock вот так:
    pub async fn apply_updates(
        &self,
        updated: &[SkinMeta],
        client: &Client,
    ) -> Result<(), Error> {
        let semaphore = Arc::new(Semaphore::new(10));
        let mut join_set = JoinSet::new();
        let mut errors = vec![];

        for skin in updated.iter().cloned() {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let client = client.clone();
            let lock = self.inner.clone();
            let save_path = self.path.clone().join(&skin.name).with_extension("png");
            join_set.spawn(async move {
                let result = match client.get(&skin.origin).send().await {
                    Ok(resp) => match resp.bytes().await {
                        Ok(bytes) => {
                            match fs::File::create(&save_path).await {
                                Ok(mut file) => {
                                    file.write_all(&bytes).await?;
                                }
                                Err(e) => {
                                    return Err(Error::SaveFailed {
                                        path: save_path,
                                        name: skin.name.clone(),
                                        error: e.to_string(),
                                    });
                                }
                            }

                            let existed = lock.insert(
                                skin.name.clone(),
                                LockMeta {
                                    path: save_path.canonicalize().unwrap().display().to_string(),
                                    origin: skin.origin.clone(),
                                    ita: skin.ita.clone(),
                                },
                            );

                            match existed {
                                Some(e) => info!(name=%skin.name, path=%e.path, "🔄 Replaced skin"),
                                None => info!(name=%skin.name, "🆕 Added new skin"),
                            }

                            Ok(())
                        }
                        Err(e) => Err(Error::DownloadFailed {
                            name: skin.name.clone(),
                            error: e.to_string(),
                        }),
                    },
                    Err(e) => Err(Error::DownloadFailed {
                        name: skin.name.clone(),
                        error: e.to_string(),
                    }),
                };
                drop(permit);
                result
            });
        }

        // Собираем результаты всех задач
        while let Some(res) = join_set.join_next().await {
            if let Err(e) = res {
                error!("⚠️ JoinError: {e:?}");
            } else if let Ok(Err(err)) = res {
                error!("⚠️ {:?}", err);
                errors.push(err);
            }
        }

        self.save().await?;

        if !errors.is_empty() {
            for e in &errors {
                error!("❌ {:?}", e);
            }
        }

        Ok(())
    }
}
