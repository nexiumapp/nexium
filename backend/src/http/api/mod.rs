use actix_web::{web, Scope};

use super::not_found;

mod account;
mod health;

/// Returns the routes of this scope.
pub fn routes() -> Scope {
    web::scope("/api")
        .service(account::routes())
        .service(health::routes())
        .default_service(web::route().to(not_found))
}
