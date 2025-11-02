use std::sync::Arc;

use ohkami::{Json, fang::Context};

use crate::{app::AppState, error::Error};

#[inline(always)]
/// Represent GET method to return list of skins
pub async fn lock_handler(
    Context(state): Context<'_, Arc<AppState>>
) -> Result<Json<Vec<String>>, Error> {
    Ok(Json(
        state
            .lock
            .store
            .iter()
            .map(|x| x.key().to_string())
            .collect::<Vec<_>>(),
    ))
}
