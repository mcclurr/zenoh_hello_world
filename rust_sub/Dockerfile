# -------- build --------
FROM rust:1.88-slim AS builder
WORKDIR /src

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.toml
COPY src/ src/

RUN cargo build --release

# -------- runtime --------
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /src/target/release/rust_sub /app/rust_sub
RUN mkdir -p /app/out

CMD ["/app/rust_sub"]
