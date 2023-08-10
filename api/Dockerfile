FROM rust:latest as builder
ENV SQLX_OFFLINE=true
WORKDIR /api

COPY . .

RUN rustup target add x86_64-unknown-linux-gnu 
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM debian:buster-slim
COPY --from=builder /api/target/x86_64-unknown-linux-gnu/release/api /api
ENTRYPOINT [ "/api" ]
EXPOSE 3636
