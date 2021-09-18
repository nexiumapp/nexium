use rocket::Route;

mod new;

/// Return all routes for the accounts route.
pub fn routes() -> Vec<Route> {
    routes![new::route]
}
