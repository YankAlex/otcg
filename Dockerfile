FROM rust:1.97.1-alpine as rust-clang

#RUN ["apt-get", "update"]
#RUN ["apt-get", "install", "clang", "--no-install-recommends", "-y"]

FROM rust-clang as depecdecies

WORKDIR /app

COPY ./Cargo.toml ./
COPY ./engine/Cargo.toml ./engine/Cargo.toml
COPY ./game_server/Cargo.toml ./game_server/Cargo.toml
COPY ./lobby_server/Cargo.toml ./lobby_server/Cargo.toml

RUN ["cargo", "update", "--release"]

FROM rust-clang as builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./engine ./engine
COPY ./game_server ./game_server
COPY ./lobby_server ./lobby_server

ENV RUST_LOG=otcg=trace

RUN ["cargo", "build", "--release"]

FROM node:26.5.0-alpine3.23 AS frontend-builder

WORKDIR /app

COPY ./client .

RUN npm i
RUN npm run build

FROM nginx:1.29.0-alpine3.22 AS server

WORKDIR /app

COPY --from=builder /app/target/release/otcg ./otcg
COPY --from=frontend-builder /app/dist /frontend
COPY ./nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["sh", "-c", "nginx -g 'daemon off;' & RUST_LOG=trace RUST_BACKTRACE=1 GAME=unmatched /app/otcg"]
