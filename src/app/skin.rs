use std::path::PathBuf;

use image::ImageFormat;
use ohkami::{
    IntoResponse, Ohkami, Query, Route,
    claw::status::Created,
    openapi::{self, Schema},
    serde::Deserialize,
};
use tee_morphosis::tee::{Tee, hsl::ddnet_color_to_hsl, parts::TeePart, skin::TEE_SKIN_LAYOUT};
use tokio::{fs, task::spawn_blocking};
use tracing::info;

use crate::{
    app::{logger::LogRequest, png::Png},
    error::Error,
};

pub fn skin_router() -> Ohkami {
    Ohkami::new((LogRequest, openapi::Tag("skin"), "/".GET(skin_handler)))
}

#[derive(Debug, Deserialize, Schema)]
pub struct SkinQuery<'req> {
    pub body: Option<u32>,
    pub feet: Option<u32>,
    pub name: Option<&'req str>,
}

async fn skin_handler(Query(query): Query<SkinQuery<'_>>) -> Result<impl IntoResponse, Error> {
    let path = PathBuf::from("./.ref")
        .join(query.name.ok_or(Error::QueryNameNotFound)?)
        .with_extension("png");
    info!(path=%path.display());

    let uv = fs::read(path.clone()).await.map_err(Error::Io)?;

    let tee = spawn_blocking(move || {
        Tee::new(uv.into(), ImageFormat::Png).map(|mut tee| {
            if let Some(value) = query.body {
                tee.apply_hsv_to_parts(
                    ddnet_color_to_hsl(value),
                    &[TeePart::Body, TeePart::BodyShadow],
                );
            }
            if let Some(value) = query.feet {
                tee.apply_hsv_to_parts(
                    ddnet_color_to_hsl(value),
                    &[TeePart::Feet, TeePart::FeetShadow],
                );
            }
            tee.compose_default(TEE_SKIN_LAYOUT)
        })
    })
    .await???;
    Ok(Created(Png(tee.into())))
}
