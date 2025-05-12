FROM rust

WORKDIR /usr/src/myapp

COPY Cargo.toml Cargo.toml
COPY src/lib.rs src/lib.rs

RUN cargo fetch

COPY src src
COPY data data

RUN cargo build -r --offline


FROM debian:bookworm-slim

WORKDIR /usr/src/myapp

RUN apt update && apt install -y libssl3 ca-certificates

COPY --from=0 /usr/src/myapp/target/release/bot-rs /usr/local/bin/bot-rs

CMD bot-rs
