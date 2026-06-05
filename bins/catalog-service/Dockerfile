FROM rust:1.93-slim AS builder

RUN apt-get update && apt-get install -y cmake build-essential libssl-dev pkg-config libcurl4-openssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .sqlx .sqlx
COPY bins/catalog-service bins/catalog-service
COPY crates crates

ENV SQLX_OFFLINE=true
RUN cargo build --release --bin catalog-service

FROM debian:trixie-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/catalog-service /usr/local/bin/catalog-service

EXPOSE 3000

CMD ["catalog-service"]
