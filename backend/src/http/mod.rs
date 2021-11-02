use actix_redis::{RedisSession, SameSite};
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};
use sqlx::{Pool, Postgres};
use time::Duration;

use crate::environment::Environment;

mod api;
mod extractors;
mod frontend;
mod helpers;

pub use extractors::*;
pub use helpers::*;

/// Start the http server.
pub async fn start(conn: Pool<Postgres>, env: Environment) -> std::io::Result<()> {
    info!("Starting HTTP service");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(conn.clone()))
            .app_data(Data::new(env.clone()))
            .wrap(
                RedisSession::new(env.redis_url.clone(), env.secret.as_bytes())
                    .cookie_name("nexium")
                    .ttl(7 * 24 * 60 * 60) // Keep a session active for one week.
                    .cookie_max_age(Some(Duration::days(100 * 365))) // Let the client store the session for a long time, or 100 years, whatever comes first.
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Strict),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(api::routes())
            .service(web::resource("/{_:.*}").route(web::get().to(|p| frontend::dist(&p))))
            .default_service(web::get().to(frontend::index))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
