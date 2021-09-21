use rocket::Route;

mod new;
mod whoami;

/// Return all routes for the accounts route.
pub fn routes() -> Vec<Route> {
    routes![new::route, whoami::route]
}
