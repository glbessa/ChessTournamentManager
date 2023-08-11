FROM rust:latest as builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/frontend
COPY ./frontend .

RUN trunk build --release

WORKDIR /usr/src/website

ENV SQLX_OFFLINE=true

COPY --from=builder /usr/src/frontend/dist ./static
COPY . .

RUN rustup override set nightly
RUN rustup target add x86_64-unknown-linux-gnu 
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM debian:bullseye-slim

COPY --from=builder /usr/src/api/target/x86_64-unknown-linux-gnu/release/website /usr/local/bin

ENV ROCKET_ENV=prod
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=3000
ENV ROCKET_LOG=critical

ENTRYPOINT [ "/usr/local/bin/website" ]

EXPOSE 3000
