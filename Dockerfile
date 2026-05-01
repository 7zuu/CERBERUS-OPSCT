FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/cerberus-cli /usr/local/bin/cerberus-cli

ENTRYPOINT ["cerberus-cli"]
