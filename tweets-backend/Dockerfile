FROM twote-rust-builder AS chef
WORKDIR /app

# Build the service
FROM chef AS builder
COPY .sqlx/ /app/.sqlx/
COPY common/ /app/common/
COPY schemas/ /app/schemas/
COPY tweets-backend/ /app/tweets-backend/
RUN cargo build --bin tweets-backend

# Run the binary from a the runtime image
FROM twote-rust-runtime

COPY --from=builder /app/target/debug/tweets-backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/tweets-backend"]