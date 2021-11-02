use std::borrow::Cow;

use actix_web::{body::Body, web, HttpResponse};
use mime_guess::from_path;
use rust_embed::RustEmbed;

/// Embedded frontend files for serving from a single binary.
#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Frontend;

/// Handle an request for a static file.
fn handle(path: &str) -> HttpResponse {
    match Frontend::get(path) {
        Some(content) => {
            let body: Body = match content.data {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => match path {
            "index.html" => HttpResponse::NotFound().body("404 Not Found"),
            _ => handle("index.html"),
        },
    }
}

/// Serve the index as a fallback.
pub fn index() -> HttpResponse {
    handle("index.html")
}

/// Serve the static frontend files, like CSS and JS.
pub fn dist(path: &web::Path<String>) -> HttpResponse {
    handle(path.as_str())
}
