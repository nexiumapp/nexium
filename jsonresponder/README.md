# JsonResponder

This is an helper crate for the backend, which provides the `JsonResponder` macro, to easily derive errors in JSON format for Rocket.

The format is as follows:

```json
{
    "code": "{responsecode}",
    "error": "{readableerror}"
}
```

Where:

-   `responsecode` is an machine readable code, this is from an mandantory `code()` function in the implemented struct.
-   `readableerror` is an error for humans, often derrived using the `thiserror` crate.

## Example

To create an error enum, make something like this:

```rust
#[derive(Error, Debug, JsonResponder)]
pub enum RouteError {
    #[error("Access denied.")]
    AccessDenied,
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}

impl<'a> RouteError {
    fn code(&self) -> &'a str {
        match self {
            RouteError::AccessDenied => "accessdenied",
            RouteError::DatabaseError(_) => "databaseerror",
        }
    }
}

impl From<RouteError> for Status {
    fn from(err: RouteError) -> Self {
        match err {
            RouteError::AccessDenied => Status::Unauthorized,
            RouteError::DatabaseError(_) => Status::InternalServerError,
        }
    }
}
```

Now you can use this in routes like this:

```rust
#[get("/whoami")]
pub async fn route(
    pool: &State<Pool<Postgres>>,
) -> Result<Json<Response>, RouteError> {
    let mut conn = pool.acquire().await?;
    let account = logic::account::Account::find(&mut conn, "some-name").await?;

    Ok(Json(Response { account }))
}
```

If the two called functions will either return `RouteError` or `RouteError`, then the responder will automatically convert the error to valid JSON.
For example, when the pool fails to acquire an connection, the response will be something like:

```json
{
    "code": "databaseerror",
    "error": "An internal database error occured."
}
```

Because `RouteError` has an `From` implementation into `Status`, the correct status will also be set to an `500 internal server error`.
