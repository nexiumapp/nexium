use actix_session::Session;
use actix_web::{post, web::Json};
use serde::Serialize;
use uuid::Uuid;

use crate::http::UserGuard;

/// Log out the current user.
#[post("/logout")]
async fn logout(_uuid: UserGuard<Uuid>, session: Session) -> Json<Response> {
    // Purge the session.
    session.purge();

    Json(Response {})
}

/// Success response of this route.
#[derive(Serialize)]
struct Response {}
