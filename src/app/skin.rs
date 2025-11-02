use std::{fmt::Display, sync::Arc};

use ohkami::{
    Ohkami, Query, Route,
    claw::status::OK,
    fang::Context,
    openapi::{self, Schema, SchemaRef, operation},
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

#[derive(Debug, Clone, Deserialize, Hash, PartialEq, Eq, Serialize)]
/// Base/Default/Main skin query
pub struct SkinQuery {
    /// **note**: Replace all whitespaces to `_`
    pub name: String,
    /// DDNet value
    pub body: Option<u32>,
    /// DDNet value
    pub feet: Option<u32>,
}

impl Schema for SkinQuery {
    fn schema() -> impl Into<SchemaRef> {
        openapi::component(
            "SkinQuery",
            openapi::object()
                .property(
                    "name",
                    openapi::string()
                        .format("a-zA-Z0-9_")
                        .description("Skin name")
                        .example("zzz"),
                )
                .optional(
                    "body",
                    openapi::string()
                        .description("DDNet value")
                        .example("32132114")
                        .nullable(),
                )
                .optional(
                    "feet",
                    openapi::string()
                        .description("DDNet value")
                        .example("32132114")
                        .nullable(),
                ),
        )
    }
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
#[instrument(skip_all, level="info", fields(name=%query.name, body=?query.body, feet=?query.feet))]
#[operation({
    summary: "Get rendered skin image",
})]
/// Represent GET method to return a builded skin by query
async fn skin_handler(
    Context(state): Context<'_, Arc<AppState>>,
    Query(query): Query<SkinQuery>,
) -> Result<OK<Png>, Error> {
    Ok(OK(Png(match state.cache.get(&query).await {
        Ok(Some(e)) => e.to_vec(),
        _ => state.lock.get(state.cache.clone(), query).await?,
    })))
}
