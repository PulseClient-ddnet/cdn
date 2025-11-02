pub mod cache;
pub mod lock;
pub mod logger;
pub mod png;
pub mod skin;

use std::sync::Arc;

use ohkami::{
    Ohkami, Route,
    claw::{content::Html, status},
    fang::Context,
    openapi,
};
use tokio::fs;

const DOC_HTML_TEMPLATE: &str = include_str!("../.static/scalar.html");
const DOC_HTML_PATH: &str = "./static/doc.html";

use crate::{app::skin::skin_router, cache::Cache, rsync::lock::Lock};
#[inline]
async fn health_check() -> status::NoContent {
    status::NoContent
}

async fn doc() -> Html {
    Html(fs::read_to_string(DOC_HTML_PATH).await.expect("wtf"))
}

pub struct AppState {
    pub lock: Lock,
    pub cache: Cache,
}

pub async fn app(
    lock: Lock,
    cache: Cache,
    address: &str,
) {
    let router = Ohkami::new((
        Context::new(Arc::new(AppState {
            lock,
            cache,
        })),
        "/skin".By(skin_router()),
        "/uvs".GET(health_check),
        "/health".GET(health_check),
    ));

    let bytes = router.__openapi_document_bytes__(openapi::OpenAPI {
        title: "DDNET Tee generator",
        version: "1",
        servers: &[],
    });
    fs::write(
        DOC_HTML_PATH,
        DOC_HTML_TEMPLATE.replace("$spec", &String::from_utf8(bytes).unwrap()),
    )
    .await
    .expect("wtf");

    let router = Ohkami::new(("/doc".GET(doc), "/".By(router)));
    router.howl(address).await;
}
