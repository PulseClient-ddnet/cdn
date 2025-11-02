use std::sync::Arc;

use ohkami::{IntoResponse, Json, fang::Context};

use crate::{app::AppState, error::Error};

#[inline(always)]
/// Represent GET method to return list of skins
pub async fn lock_handler<'a>(
    Context(state): Context<'a, Arc<AppState<'a>>>
) -> Result<impl IntoResponse + 'a, Error> {
    Ok(Json(serde_json::to_string_pretty::<Vec<String>>(
        state
            .lock
            .store
            .iter()
            .map(|x| x.key().to_string())
            .collect::<Vec<_>>()
            .as_ref(),
    )?))
}
