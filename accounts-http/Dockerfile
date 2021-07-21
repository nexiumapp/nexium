# Load the base image to copy from.
FROM nexium-base as base

# Build from the slim image.
FROM debian:buster-slim

# Copy the binary from the base image.
COPY --from=base /nexium/target/release/accounts .

# Port to serve on.
EXPOSE 50051

# Start the binary.
CMD ["./accounts"]
