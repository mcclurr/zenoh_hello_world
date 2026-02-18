# -------- planner --------
FROM rust:1.88-slim AS planner
WORKDIR /src

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Install cargo-chef (you can pin a version if you want)
RUN cargo install cargo-chef

# Copy full workspace (since build context is ./rust_sub, this stays small)
COPY . .

# Generate the dependency "recipe"
RUN cargo chef prepare --recipe-path recipe.json


# -------- builder --------
FROM rust:1.88-slim AS builder
WORKDIR /src

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config ca-certificates \
 && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef

# Cook dependencies (this layer becomes cacheable even when src changes)
COPY --from=planner /src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Now copy the full source and build your binary
COPY . .
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
