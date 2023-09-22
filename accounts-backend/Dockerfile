FROM lukemathwalker/cargo-chef:latest-rust-1.72 AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml /app/Cargo.toml
COPY common/Cargo.toml /app/common/Cargo.toml
COPY schemas/Cargo.toml /app/schemas/Cargo.toml
COPY twote-api/Cargo.toml /app/twote-api/Cargo.toml
COPY accounts-backend/Cargo.toml /app/accounts-backend/Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Install protoc to compile protobuf files
RUN apt update && apt install -y protobuf-compiler
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json
# Build application
COPY common/ /app/common/
COPY schemas/ /app/schemas/
COPY accounts-backend/ /app/accounts-backend/
RUN cargo build --bin accounts-backend

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm AS runtime
WORKDIR /app

# Install wget to download grpc-health-probe
RUN apt update && apt install -y wget \
    && wget -qO/usr/local/bin/grpc-health-probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/v0.4.5/grpc_health_probe-linux-amd64 \
    && chmod +x /usr/local/bin/grpc-health-probe

COPY --from=builder /app/target/debug/accounts-backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/accounts-backend"]