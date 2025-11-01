use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;

use crate::{error::Error, rsync::lock::LockMeta};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct SkinMeta {
    /// URL
    pub origin: String,
    /// Name of the skin
    pub name: String,
    /// Last update timestamp
    pub ita: String,
}

impl SkinMeta {
    pub fn eq_lock_meta(
        &self,
        meta: &LockMeta,
    ) -> bool {
        self.ita == meta.ita && self.origin == meta.origin
    }
}

/// Парсит страницу директории и возвращает список ссылок на `.png` файлы
pub async fn fetch_skin_list(
    client: &Client,
    url: &str,
) -> Result<Vec<SkinMeta>, Error> {
    let text = client.get(url).send().await?.text().await?;
    let url = url.to_string();
    // пример строки:
    // <a href="skin.png">skin.png</a> 2024-12-12 10:15  12345
    let re = Regex::new(
        r#"<a href="([^"/]+\.png)">[^<]+</a>\s+(\d{2}-[A-Za-z]{3}-\d{4}\s+\d{2}:\d{2})"#,
    )
    .unwrap();

    let list = spawn_blocking(move || {
        re.captures_iter(&text)
            .par_bridge()
            .map(|cap| {
                let name = &cap[1];
                let modified = &cap[2];

                SkinMeta {
                    origin: format!("{}{}", url, name),
                    // Collect the character iterator into a String for the name field
                    name: name.chars().take(name.len() - 4).collect::<String>(),
                    ita: modified.to_string(),
                }
            })
            .collect::<Vec<SkinMeta>>()
    })
    .await?;

    Ok(list)
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use crate::rsync::parser::{SkinMeta, fetch_skin_list};

    #[tokio::test]
    async fn parsing() {
        let client = Client::new();
        let mut links: Vec<SkinMeta> = vec![];
        let urls = vec![
            "https://ddnet.org/skins/skin/",
            "https://ddnet.org/skins/skin/community/",
        ];
        for url in urls {
            let mut local_links = fetch_skin_list(&client, &url).await.unwrap();
            println!("{url} -> len PNGs: {:?}", local_links.len());
            links.append(&mut local_links);
        }
        println!("PNG links: {:#?}", links.len());
        println!("PNGs: {:#?}", links);
        assert!(!links.is_empty());
    }
}
