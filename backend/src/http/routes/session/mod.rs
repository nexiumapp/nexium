use rocket::Route;

mod refresh;

/// Return all routes for the session route.
pub fn routes() -> Vec<Route> {
    routes![refresh::route]
}
