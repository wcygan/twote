FROM twote-rust-builder AS chef
WORKDIR /app

FROM chef AS builder
COPY .sqlx/ /app/.sqlx/
COPY common/ /app/common/
COPY schemas/ /app/schemas/
COPY accounts-backend/ /app/accounts-backend/
RUN cargo build --bin accounts-backend

FROM twote-rust-runtime

COPY --from=builder /app/target/debug/accounts-backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/accounts-backend"]