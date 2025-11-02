use std::sync::Arc;

use ohkami::{Json, fang::Context};
use tracing::debug;

use crate::{
    app::{AppState, skin::SkinQuery},
    error::Error,
};

#[inline(always)]
/// Represent GET method to return list of skins
pub async fn cache_handler(
    Context(state): Context<'_, Arc<AppState>>
) -> Result<Json<Vec<SkinQuery>>, Error> {
    Ok(Json(
        state
            .cache
            .store
            .iter()
            .map(|x| {
                let key = x.key().clone();
                debug!(key=?key);
                key
            })
            .collect::<Vec<_>>(),
    ))
}
