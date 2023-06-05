FROM rust:1.69 as build

WORKDIR /build

COPY Cargo.* ./
COPY src ./src

RUN cargo build --release --target-dir /publish

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /publish/release/ddns-agent-domainsgoogle /ddns-agent-domainsgoogle

RUN chmod +x /ddns-agent-domainsgoogle

ENTRYPOINT ["/ddns-agent-domainsgoogle"]
