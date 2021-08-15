extern crate log;
#[macro_use]
extern crate rocket;

mod http;

fn main() {
    env_logger::init();

    let runtime = tokio::runtime::Runtime::new().unwrap();

    // Start the HTTP server.
    runtime.block_on(http::start());
}
