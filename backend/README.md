# Nexium Backend

This is the backend for Nexium.
It is written in Rust and exposes both the HTTP and the SMTP service.
For it to run (or compile!) it requires an running PostgreSQL server.

## How to Run

First start an PostgreSQL server.
The easiest way to do that is to run it via docker:

```bash
docker run -p 5432:5432 --rm -e POSTGRES_PASSWORD=nexium -e POSTGRES_DB=nexium postgres:13
```

Now update the `.env` file in this folder to reflect the URL of the database.
When running it locally the current URL should work.

If you are compiling the backend you'll need to run the migration first.
This can be done by installing the Sqlx cli and running the migration.
It is not required when running the binary itself, as the migrations are embedded and ran on startup.

```bash
cargo install sqlx-cli
sqlx migrate run
```

Now the backend can be started:

```bash
cargo run
```
