FROM rust:1.93-slim AS builder

RUN apt-get update && apt-get install -y cmake build-essential libssl-dev pkg-config libcurl4-openssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY bins/basket-service bins/basket-service
COPY crates crates

RUN cargo build --release --bin basket-service

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/basket-service /usr/local/bin/basket-service

EXPOSE 3001

CMD ["basket-service"]
