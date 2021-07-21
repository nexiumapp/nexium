FROM rust:1.53

# Switch workdir.
WORKDIR /nexium

# Add additional rust components.
RUN rustup component add rustfmt

# Create filler projects.
RUN cargo new accounts
RUN cargo new accounts-http

# Copy over manifest files.
COPY ./Cargo.toml ./Cargo.toml
COPY ./accounts/Cargo.toml ./accounts/Cargo.toml
COPY ./accounts-http/Cargo.toml ./accounts-http/Cargo.toml

# Copy over the build & protocol files.
COPY ./accounts/build.rs ./accounts/build.rs
COPY ./accounts/accounts.proto ./accounts/accounts.proto

# Build dependency for release.
RUN cargo build --release

# Copy over the rest of the files.
COPY ./accounts ./accounts
COPY ./accounts-http ./accounts-http

# Remove dependencies.
RUN rm ./target/release/deps/accounts*

# Build it all for release.
RUN cargo build --release
