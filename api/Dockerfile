FROM rust:latest as builder
ENV SQLX_OFFLINE=true

WORKDIR /usr/src/api

COPY . .

RUN rustup override set nightly
RUN rustup target add x86_64-unknown-linux-gnu 
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM debian:bullseye-slim

COPY --from=builder /usr/src/api/target/x86_64-unknown-linux-gnu/release/api /usr/local/bin

ENV ROCKET_ENV=prod
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=3000
ENV ROCKET_LOG=critical

ENTRYPOINT [ "/usr/local/bin/api" ]
