use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde_json::json;
use std::io::Cursor;
use thiserror::Error;

/// Possible error codes returned to the client.
#[derive(Error, Debug)]
pub enum DelayErrors {
    #[error("Delay of {0} is less than the required 5.")]
    LowDelay(u64),
}

impl<'a> DelayErrors {
    fn code(&self) -> &'a str {
        match self {
            DelayErrors::LowDelay(_) => "lowdelay",
        }
    }
}

impl Into<Status> for DelayErrors {
    fn into(self) -> Status {
        match self {
            Self::LowDelay(_) => Status::BadRequest,
        }
    }
}

impl<'r> Responder<'r, 'static> for DelayErrors {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let res = json!({
            "code": self.code(),
            "error": self.to_string()
        })
        .to_string();

        Response::build()
            .status(self.into())
            .header(ContentType::JSON)
            .sized_body(res.len(), Cursor::new(res))
            .ok()
    }
}
