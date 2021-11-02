use std::sync::Arc;

use async_trait::async_trait;
use postbus::{Handler, SmtpService, SmtpState};
use sqlx::{Pool, Postgres};

/// Start the SMTP server.
pub async fn start(db: Pool<Postgres>) {
    let service = SmtpService::create(
        "0.0.0.0:2525".parse().unwrap(),
        "Nexium Relay".into(),
        Arc::new(SmtpHandler { _db: db }),
    );

    service.listen().await;
}

struct SmtpHandler {
    _db: Pool<Postgres>,
}

#[async_trait]
impl Handler for SmtpHandler {
    /// Validate the recipient.
    async fn recipient_local(&self, recipient: &postbus::command::Mailbox) -> bool {
        println!("Checking recipient {:#?}.", recipient);

        recipient.domain == "nexium.app".into()
    }

    /// Save the received email into the database.
    async fn save(&self, state: &SmtpState) -> bool {
        println!("Saving mail state:\r\n{:#?}", state);

        let parsed = mailparse::parse_mail(state.data.as_bytes()).unwrap();
        println!("Body:\r\n{:#?}", parsed);

        true
    }
}
