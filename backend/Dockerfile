FROM rust:1.53 as build

# Switch workdir.
WORKDIR /nexium

# Add additional rust components.
RUN rustup component add rustfmt

# Copy over source files.
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# Build it all for release.
RUN cargo build --release

# Build from the slim image.
FROM debian:buster-slim

# Copy the binary from the base image.
COPY --from=build /nexium/target/release/nexium .

# Port to serve on.
EXPOSE 8000

# Start the binary.
CMD ["./nexium"]
