use actix_web::{body::AnyBody, http::header, HttpResponse, ResponseError};
use serde::Serialize;

/// Not found route, used as a fallback when no route matches.
pub fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("404 Not Found")
}

/// Error response when the route fails and returns an API error.
/// Used by the trait below.
#[derive(Serialize)]
pub struct JsonError<'a> {
    code: &'a str,
    message: String,
}

/// API error trait, this should be implemented by all error objects.
pub trait ApiError<'a>: ResponseError {
    /// Convert the error into an unique code usable by the client.
    fn error_code(&self) -> &'a str;

    /// Convert the error into a proper JSON `HttpResponse`.
    /// This can be used in the implementation of `ResponseError` like this:
    ///
    /// ```
    /// fn error_response(&self) -> HttpResponse {
    ///     self.json()
    /// }
    /// ```
    fn json(&self) -> HttpResponse {
        let mut res = HttpResponse::new(self.status_code());

        res.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json; charset=utf-8"),
        );

        res.set_body(AnyBody::from_message(
            serde_json::to_string(&JsonError {
                code: self.error_code(),
                message: self.to_string(),
            })
            .expect("Serialization failed, which should not be impossible."),
        ))
    }
}
