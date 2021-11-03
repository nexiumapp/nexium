use actix_web::{web, Scope};

mod login;
mod logout;
mod new;
mod whoami;

/// Returns the routes of this scope.
pub fn routes() -> Scope {
    web::scope("/account")
        .service(login::login)
        .service(logout::logout)
        .service(new::new_account)
        .service(whoami::whoami)
        .default_service(web::route().to(super::not_found))
}
