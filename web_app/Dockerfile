FROM rust:lastest

RUN rustup target add wasm32-unknown-unkown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/web_app
COPY . .

RUN cd web_app && trunk build --release
RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /usr/src/web_app/target/release/web_app /usr/local/bin/web_app

WORKDIR /usr/local/bin
ENTRYPOINT [ "web_app" ]
EXPOSE 80