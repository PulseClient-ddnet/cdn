use ohkami::{FangAction, Request, Response};

#[derive(Clone)]
pub struct LogRequest;
impl FangAction for LogRequest {
    async fn fore<'a>(
        &'a self,
        req: &'a mut Request,
    ) -> Result<(), Response> {
        tracing::debug!("\nGot request: {req:#?}");
        Ok(())
    }
}
