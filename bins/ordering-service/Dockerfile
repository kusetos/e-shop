FROM rust:1.93-slim AS builder

RUN apt-get update && apt-get install -y cmake build-essential libssl-dev pkg-config libcurl4-openssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .sqlx .sqlx
COPY bins/ordering-service bins/ordering-service
COPY crates crates

ENV SQLX_OFFLINE=true
RUN cargo build --release --bin ordering-service

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ordering-service /usr/local/bin/ordering-service

EXPOSE 3002

CMD ["ordering-service"]
