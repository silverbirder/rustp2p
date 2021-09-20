FROM rust:1.48.0-alpine

WORKDIR /app/

COPY . .

RUN apk update & \
    apk add vim alpine-sdk libressl-dev & \
    cargo build