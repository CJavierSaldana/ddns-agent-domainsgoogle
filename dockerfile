# Start from a Rust base image
FROM rust:1.57 as build

# Create a new directory for the application
WORKDIR /build

# Copy all local files into the container
COPY . .

# Build the application for release with musl target
RUN cargo build --release --target-dir /publish

# Start a new stage from scratch
FROM scratch

# Copy over the binary from the build stage
COPY --from=build /publish /ddns-agent-domainsgoogle

# Set the startup command to run the binary
CMD ["/ddns-agent-domainsgoogle"]
