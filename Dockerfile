FROM rust:latest

WORKDIR /usr/src/app

COPY . .

RUN cargo check
RUN cargo test
