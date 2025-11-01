use std::borrow::Cow;

use ohkami::{claw::content::IntoContent, openapi};

/// Represents a PNG image content.
pub struct Png(pub Vec<u8>);

impl IntoContent for Png {
    const CONTENT_TYPE: &'static str = "image/png";

    fn into_content(self) -> Result<std::borrow::Cow<'static, [u8]>, impl std::fmt::Display> {
        Result::<_, std::convert::Infallible>::Ok(Cow::Owned(self.0))
    }

    fn openapi_responsebody() -> impl Into<openapi::schema::SchemaRef> {
        openapi::schema::SchemaRef::Reference("png")
    }
}
