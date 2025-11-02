use std::sync::Arc;

use ohkami::{IntoResponse, Json, fang::Context};
use tracing::debug;

use crate::{app::AppState, error::Error};

#[inline(always)]
/// Represent GET method to return list of skins
pub async fn cache_handler<'a>(
    Context(state): Context<'a, Arc<AppState>>
) -> Result<impl IntoResponse + 'a, Error> {
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
