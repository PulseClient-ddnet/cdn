use std::borrow::Cow;

use ohkami::{
    claw::content::IntoContent,
    openapi::{self, Schema},
};

/// Represents a PNG image content.
#[derive(Debug, Schema)]
#[openapi(component)]
pub struct Png(pub Vec<u8>);

impl IntoContent for Png {
    const CONTENT_TYPE: &'static str = "image/png";

    #[inline(always)]
    fn into_content(self) -> Result<std::borrow::Cow<'static, [u8]>, impl std::fmt::Display> {
        Result::<_, std::convert::Infallible>::Ok(Cow::Owned(self.0))
    }

    #[inline(always)]
    fn openapi_responsebody() -> impl Into<openapi::schema::SchemaRef> {
        openapi::string().format("binary")
    }
}
