FROM rust:1.93-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY bins/identity-service bins/identity-service
COPY crates crates

ENV SQLX_OFFLINE=true
RUN cargo build --release --bin identity-service

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/identity-service /usr/local/bin/identity-service

EXPOSE 3003

CMD ["identity-service"]
