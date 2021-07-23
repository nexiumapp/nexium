# Load the base image to copy from.
FROM nexium-base:base as base

# Build from the slim image.
FROM debian:buster-slim

# Copy the binary from the base image.
COPY --from=base /nexium/target/release/accounts-http .

# Port to serve on.
EXPOSE 8000

# Start the binary.
CMD ["./accounts-http"]
