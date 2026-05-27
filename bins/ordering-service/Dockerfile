FROM rust:1.93-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY bins/ordering-service bins/ordering-service
COPY crates crates

ENV SQLX_OFFLINE=true
RUN cargo build --release --bin ordering-service

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ordering-service /usr/local/bin/ordering-service

EXPOSE 3002

CMD ["ordering-service"]
