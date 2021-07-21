use tonic::transport::Channel;

pub mod error;
pub mod proto;

/// Client used to communicate with the service.
pub struct Client {
    conn: proto::accounts_client::AccountsClient<Channel>,
}

impl Client {
    /// Connect to the accounts service.
    pub async fn connect(url: &'static str) -> Result<Client, Box<dyn std::error::Error>> {
        let conn = proto::accounts_client::AccountsClient::connect(url).await?;

        Ok(Client { conn: conn })
    }

    /// Send a ping request to the service.
    /// This requires a `seconds` parameter, which indicates how long to pause for.
    pub async fn ping(&self, seconds: u64) -> Result<(), error::PingError> {
        let request = tonic::Request::new(proto::accounts_proto::PingRequest { seconds: seconds });
        let res = self.conn.clone().ping(request).await;

        match res {
            Ok(_) => Ok(()),
            Err(status) => match status.code() {
                tonic::Code::Internal => {
                    let decoded: error::PingError = serde_json::from_slice(status.details())
                        .unwrap_or(error::PingError::RPCError(String::from(
                            "Failed to decode message.",
                        )));

                    return Err(decoded);
                }
                _ => Err(error::PingError::RPCError(status.to_string())),
            },
        }
    }
}
