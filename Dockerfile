FROM rust:1.60 as build

RUN USER=root cargo new --bin axum-server
WORKDIR /axum-server

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/axum-server*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /holy/target/release/axum-server .

CMD ["./axum-server"]