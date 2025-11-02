use std::sync::Arc;

use ohkami::{
    IntoResponse, Ohkami, Query, Route,
    claw::status::Created,
    fang::Context,
    openapi::{self, Schema},
    serde::Deserialize,
};

use crate::{
    app::{AppState, lock::lock_handler, logger::LogRequest, png::Png},
    error::Error,
};

#[inline(always)]
pub fn skin_router() -> Ohkami {
    Ohkami::new((
        LogRequest,
        openapi::Tag("skin"),
        "/".GET(skin_handler),
        "/store".GET(lock_handler),
    ))
}

#[derive(Debug, Clone, Copy, Deserialize, Schema, Hash, PartialEq, Eq)]
/// Params for expose Tee
pub struct SkinQuery<'req> {
    pub name: &'req str,
    pub body: Option<u32>,
    pub feet: Option<u32>,
}

#[inline(always)]
/// Represent GET method to return a builded skin by query
async fn skin_handler<'a>(
    Context(state): Context<'a, Arc<AppState<'a>>>,
    // Context(cache): Context<'a, Cache<'a>>,
    Query(query): Query<SkinQuery<'a>>,
) -> Result<impl IntoResponse + 'a, Error> {
    Ok(Created(Png(match state.cache.get(query).await {
        Ok(Some(e)) => e,
        _ => state.lock.get(state.cache.clone(), query).await?,
    })))
}
