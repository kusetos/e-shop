FROM rust:1.93-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY bins/catalog-service bins/catalog-service
COPY crates crates
COPY .sqlx .sqlx

ENV SQLX_OFFLINE=true
RUN cargo build --release --bin catalog-service

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/catalog-service /usr/local/bin/catalog-service

EXPOSE 3000

CMD ["catalog-service"]
