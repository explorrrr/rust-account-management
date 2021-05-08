# 開発環境
FROM rust:1.48-buster as develop-stage
RUN apt-get update & apt-get install git
WORKDIR /app/root
RUN cargo install cargo-watch
RUN rustup component add rust-src
RUN cargo install cargo-edit
COPY . .

# ビルド環境
FROM develop-stage as build-stage
RUN cargo build --release

# 本番環境
FROM rust:1.48-slim-buster
COPY --from=build-stage /app/root/target/release/api .
EXPOSE 8000
CMD ["/usr/local/bin/api"]
