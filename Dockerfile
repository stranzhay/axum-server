FROM rust:1.61 as builder

RUN USER=root cargo new --bin axum
WORKDIR /axum

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/axum*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /axum-server/target/release/axum .

CMD ["./axum"]

