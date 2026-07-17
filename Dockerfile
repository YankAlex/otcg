FROM rust:1.97.1-alpine as builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./engine ./engine
COPY ./game_server ./game_server
COPY ./lobby_server ./lobby_server

ENV RUST_LOG=otcg=trace

RUN ["cargo", "build", "--release"]

FROM alpine:3.24.1 AS server

WORKDIR /app

COPY --from=builder /app/target/release/otcg ./otcg

CMD ["/app/otcg"]
