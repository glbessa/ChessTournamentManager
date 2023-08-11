FROM rust:latest as builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/web_app
COPY . .

RUN trunk build --release
RUN cargo build --release

FROM nginx:stable-alpine-slim

COPY --from=builder /usr/src/web_app/dist /usr/share/nginx/html
COPY default.conf /etc/nginx/conf.d/

#CMD [ "sudo service nginx restart" ]

#EXPOSE 80