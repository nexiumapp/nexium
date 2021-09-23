use std::{borrow::Cow, path::PathBuf};

use rocket::{
    http::{ContentType, Status},
    Route,
};
use rust_embed::RustEmbed;

/// Embedded frontend files for serving from a single binary.
#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Frontend;

/// Provides the index HTML file on the root.
#[get("/")]
fn index<'r>() -> Result<(ContentType, Cow<'r, [u8]>), Status> {
    let file = match Frontend::get("index.html") {
        Some(file) => file,
        None => return Err(Status::NotFound),
    };

    Ok((ContentType::HTML, file.data))
}

/// Tries to serve the rest of the resources too.
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
pub fn routes() -> Vec<Route> {
    routes![index, subresource]
}
