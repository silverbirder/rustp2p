FROM rust:1.56.0 AS builder
RUN USER=root cargo new --bin app
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN cargo install --locked --path .
ENV GOOGLE_APPLICATION_CREDENTIALS /app/a.json
COPY ./secrets/a.json /app/a.json
RUN mkdir lake
ENTRYPOINT ["imgitor"]