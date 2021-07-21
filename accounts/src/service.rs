use crate::proto;
use std::time::Duration;
use tokio::time::sleep;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct AccountsService {}

#[tonic::async_trait]
impl proto::accounts_server::Accounts for AccountsService {
    /// handle the ping request.
    async fn ping(
        &self,
        req: Request<proto::accounts_proto::PingRequest>,
    ) -> Result<Response<proto::accounts_proto::PingResponse>, Status> {
        let seconds = req.get_ref().seconds;

        if seconds < 5 {
            return Err(crate::error::PingError::DelayTooLowError.into());
        }

        sleep(Duration::from_secs(seconds)).await;
        Ok(Response::new(proto::accounts_proto::PingResponse {}))
    }
}
