use actix_web::{web, HttpResponse, Scope};

/// Returns the routes of this scope.
pub fn routes() -> Scope {
    web::scope("/health")
        .route("", web::get().to(HttpResponse::Ok))
        .default_service(web::route().to(super::not_found))
}
