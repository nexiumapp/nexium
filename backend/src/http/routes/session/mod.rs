use rocket::Route;

mod login;
mod refresh;

/// Return all routes for the session route.
pub fn routes() -> Vec<Route> {
    routes![login::route, refresh::route]
}
