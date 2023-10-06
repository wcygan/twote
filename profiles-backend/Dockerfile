FROM twote-rust-builder AS chef
WORKDIR /app

# Build the service
FROM chef AS builder
COPY .sqlx/ /app/.sqlx/
COPY common/ /app/common/
COPY schemas/ /app/schemas/
COPY profiles-backend/ /app/profiles-backend/
RUN cargo build --bin profiles-backend

# Run the binary from a the runtime image
FROM twote-rust-runtime

COPY --from=builder /app/target/debug/profiles-backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/profiles-backend"]