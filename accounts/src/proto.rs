/// Include the account protocol.
pub use accounts_proto::{accounts_client, accounts_server};

pub mod accounts_proto {
    tonic::include_proto!("accounts");
}
