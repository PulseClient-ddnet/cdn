use ohkami::{FangAction, Request, Response};
use tracing::{info, instrument};

#[derive(Clone)]
pub struct LogRequest;
impl FangAction for LogRequest {
    #[inline(always)]
    #[instrument(skip_all, level="info", fields(ip=%req.ip, type=%req.method, user_agent=?req.headers.user_agent(), path=%req.path))]
    async fn fore<'a>(
        &'a self,
        req: &'a mut Request,
    ) -> Result<(), Response> {
        info!("got req");
        Ok(())
    }
}
