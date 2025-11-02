use std::{fmt::Display, sync::Arc};

use ohkami::{
    IntoResponse, Ohkami, Query, Route,
    claw::status::Created,
    fang::Context,
    openapi::{self, Schema},
    serde::Deserialize,
};
use serde::Serialize;
use tracing::instrument;

use crate::{
    app::{AppState, cache::cache_handler, lock::lock_handler, logger::LogRequest, png::Png},
    error::Error,
};

#[inline(always)]
pub fn skin_router() -> Ohkami {
    Ohkami::new((
        LogRequest,
        openapi::Tag("skin"),
        "/".GET(skin_handler),
        "/store".GET(lock_handler),
        "/cache".GET(cache_handler),
    ))
}

#[derive(Debug, Clone, Deserialize, Schema, Hash, PartialEq, Eq, Serialize)]
pub struct SkinQuery {
    /// Replace all whitespaces to `_`
    pub name: String,
    pub body: Option<u32>,
    pub feet: Option<u32>,
}

impl Display for SkinQuery {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("SkinQuery")
            .field("name", &self.name)
            .field("body", &self.body)
            .field("feet", &self.feet)
            .finish()
    }
}

#[inline(always)]
#[instrument(skip(state))]
/// Represent GET method to return a builded skin by query
async fn skin_handler<'a>(
    Context(state): Context<'a, Arc<AppState>>,
    Query(query): Query<SkinQuery>,
) -> Result<impl IntoResponse, Error> {
    Ok(Created(Png(match state.cache.get(&query).await {
        Ok(Some(e)) => e.to_vec(),
        _ => state.lock.get(state.cache.clone(), query).await?,
    })))
}
