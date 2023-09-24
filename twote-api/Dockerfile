FROM twote-rust-builder AS chef
WORKDIR /app

FROM chef AS builder
COPY common/ /app/common/
COPY schemas/ /app/schemas/
COPY twote-api/ /app/twote-api/
RUN cargo build --bin twote-api

FROM twote-rust-runtime

COPY --from=builder /app/target/debug/twote-api /usr/local/bin
ENTRYPOINT ["/usr/local/bin/twote-api"]