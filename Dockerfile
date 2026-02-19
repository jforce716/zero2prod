# Use a slim Rust image to build the code
FROM rust:slim AS builder

# Install system dependencies needed for compiling (OpenSSL, etc.)
RUN apt-get update && apt-get install -y pkg-config libssl-dev

WORKDIR /app
COPY . .
RUN cargo build

# Use a tiny runtime image
#FROM debian:trixie-slim
FROM debian:trixie

RUN apt-get update && apt-get install -y libssl3 ca-certificates procps net-tools curl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/debug/zero2prod /usr/local/bin/zero2prod

WORKDIR /app
COPY ./configuration.yml ./configuration.yml
ENTRYPOINT ["zero2prod"]