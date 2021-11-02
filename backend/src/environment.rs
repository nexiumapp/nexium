use dotenv::dotenv;
use std::env;

/// Get the configuration from the enviroment variables.
/// Returns a string with an textual error if this wass not possible.
pub fn get() -> Result<Environment, String> {
    let secret = try_get("NEXIUM_SECRET", None)?;
    let database_url = match try_get("NEXIUM_DATABASE_URL", None) {
        Ok(url) => url,
        Err(_) => try_get(
            "DATABASE_URL",
            Some("postgres://postgres:nexium@localhost/nexium".to_string()),
        )?,
    };
    let redis_url = try_get("NEXIUM_REDIS_URL", Some("127.0.0.1:6379".to_string()))?;

    if secret.len() < 256 {
        return Err("The secret is required to be at least 265 characters long.".to_string());
    }

    Ok(Environment {
        secret,
        database_url,
        redis_url,
    })
}

/// Try to get an environment variable by key.
/// If this does not exist it tries to return the Some of the default.
/// If the default is None it will return an string.
fn try_get(key: &str, default: Option<String>) -> Result<String, String> {
    dotenv().map_err(|_| "Failed to load dotfile.")?;

    match env::var_os(key) {
        Some(val) => match val.into_string() {
            Ok(val) => Ok(val),
            Err(_) => {
                return Err(format!(
                    "Environment variable '{}' does not contain valid ASCII.",
                    key
                ))
            }
        },
        None => match default {
            Some(default) => Ok(default),
            None => Err(format!(
                "The environmental variable '{}' is not set, but required!",
                key
            )),
        },
    }
}

#[derive(Clone)]
pub struct Environment {
    pub secret: String,
    pub database_url: String,
    pub redis_url: String,
}
