use actix_web::{get, web::Json};
use serde::Serialize;

use crate::{http::UserGuard, logic::account::Account};

/// Returns the current logged in user.
#[get("/whoami")]
async fn whoami(account: UserGuard<Account>) -> Json<Response> {
    Json(Response {
        account: account.into(),
    })
}

/// Success response of this route.
#[derive(Serialize)]
struct Response {
    account: Account,
}
