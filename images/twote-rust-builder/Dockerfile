FROM lukemathwalker/cargo-chef:latest-rust-1.72
WORKDIR /app

# Copy over the Cargo.toml files of every crate in the workspace
COPY Cargo.toml /app/Cargo.toml
COPY common/Cargo.toml /app/common/Cargo.toml
COPY schemas/Cargo.toml /app/schemas/Cargo.toml
COPY twote-api/Cargo.toml /app/twote-api/Cargo.toml
COPY accounts-backend/Cargo.toml /app/accounts-backend/Cargo.toml
COPY profiles-backend/Cargo.toml /app/profiles-backend/Cargo.toml

# Install & cache all dependencies
RUN apt update && apt install -y protobuf-compiler
RUN cargo chef prepare --recipe-path recipe.json
RUN cargo chef cook --recipe-path recipe.json