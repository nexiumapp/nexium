#[cfg(feature = "embedded-frontend")]
use std::{borrow::Cow, path::PathBuf};

#[cfg(feature = "embedded-frontend")]
use rocket::http::{ContentType, Status};
use rocket::Route;
#[cfg(feature = "embedded-frontend")]
use rust_embed::RustEmbed;

/// Embedded frontend files for serving from a single binary.
#[cfg(feature = "embedded-frontend")]
#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Frontend;

/// Provides the index HTML file on the root.
#[cfg(feature = "embedded-frontend")]
#[get("/")]
fn index<'r>() -> Result<(ContentType, Cow<'r, [u8]>), Status> {
    let file = match Frontend::get("index.html") {
        Some(file) => file,
        None => return Err(Status::NotFound),
    };

    Ok((ContentType::HTML, file.data))
}

/// Tries to serve the rest of the resources too.
#[cfg(feature = "embedded-frontend")]
#[get("/<path..>")]
fn subresource<'r>(path: PathBuf) -> Result<(ContentType, Cow<'r, [u8]>), Status> {
    let path_str = match path.to_str() {
        Some(path) => path,
        None => return Err(Status::NotFound),
    };

    let file = match Frontend::get(path_str) {
        Some(file) => file,
        None => return index(),
    };

    Ok((get_content_type(path)?, file.data))
}

/// Get the probable content type from the url.
#[cfg(feature = "embedded-frontend")]
fn get_content_type(path: PathBuf) -> Result<ContentType, Status> {
    let os_ext = match path.extension() {
        Some(e) => e.to_str(),
        None => return Err(Status::NotFound),
    };

    let ext_str = match os_ext {
        Some(s) => s,
        None => return Err(Status::NotFound),
    };

    let extension = match ContentType::from_extension(ext_str) {
        Some(ext) => ext,
        None => ContentType::Binary,
    };

    Ok(extension)
}

/// Static frontend routes.
/// Only runs when the `embedded-frontend` feature is enabled.
#[cfg(feature = "embedded-frontend")]
pub fn routes() -> Vec<Route> {
    routes![index, subresource]
}

/// Static frontend routes.
/// This is an placeholder function to provide the routes when the `embedded-frontend` feature is disabled.
#[cfg(not(feature = "embedded-frontend"))]
pub fn routes() -> Vec<Route> {
    routes![]
}
