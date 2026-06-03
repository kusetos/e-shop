FROM rust:1.93-slim AS builder

RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY bins/gateway bins/gateway
COPY crates crates

RUN cargo build --release --bin gateway

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/gateway /usr/local/bin/gateway

EXPOSE 8080

CMD ["gateway"]
