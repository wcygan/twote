# Creating a Rust + gRPC Backend

TLDR - See these PRs for how `accounts-backend` was setup:
1. [Add accounts-backend service](https://github.com/wcygan/twote/pull/4)
2. [Setup Docker Compose](https://github.com/wcygan/twote/pull/6)

## Creating the Rust Service

First, we need to create a new Rust service. We can do this by running the following command in the root directory:

```bash
cargo new accounts-backend
```

This will create a new Rust project in the `accounts-backend` directory. 

You need to add this new service to the [Workspace](../Cargo.toml) in the root directory:

```toml
[workspace]
members = [
    "common",
    "schemas",
    "twote-api",
    # Add the new service here: 
    "accounts-backend"
]
```

## Adding Dependencies

Next, you can add any dependencies to the `Cargo.toml` file. A Rust + gRPC service will likely need, at least, the following dependencies:

```toml
[dependencies]
common = { path = "../common" }
tokio.workspace=true
tonic.workspace=true
tonic-health.workspace=true
tracing.workspace=true
tracing-subscriber.workspace=true
```

The dependencies are being pulled from the [Workspace Dependencies](../Cargo.toml) in the root directory. This allows us to share versioned dependencies between services.

Tokio & Tonic are the dependencies which power the gRPC server. Tracing is used for logging.

## Create a new `Service`

Navigate to [service.rs](../common/src/service.rs) Add your service to the enum & fill out the `name` and `port` methods.

```rust
pub enum Service {
    TwoteApi,
    AccountsBackend,
}

impl Service {
    pub fn port(&self) -> u16 {
        match self {
            Service::TwoteApi => 8081,
            Service::AccountsBackend => 8082,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Service::TwoteApi => "twote-api",
            Service::AccountsBackend => "accounts-backend",
        }
    }
    
    ...
}
```

## Setup `main.rs`

Next, we need to setup the `main.rs` file. This file will contain the main entrypoint for the service. It will also contain the gRPC server.

```rust
use common::Service::AccountsBackend;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create the services
    let (_, health_service) = tonic_health::server::health_reporter();

    // Start the server
    let addr = AccountsBackend.socket_addr();
    println!("accounts-backend is running on {}", addr);
    Server::builder()
        .add_service(health_service)
        .serve(addr)
        .await?;

    Ok(())
}
```

## Healthcheck

Run the service:

```bash
cargo run --bin accounts-backend
```

`accounts-backend` runs on port `8082`; we can use a tool like [grpc-health-probe](https://github.com/grpc-ecosystem/grpc-health-probe) to check the health of the service:

```bash
grpc-health-probe -addr=localhost:8082
```

It should tell us `status: SERVING`.

## Docker Image

Create a new file in the `accounts-backend` directory called `Dockerfile`. This file will be used to build the Docker image for the service.

```Dockerfile
FROM twote-rust-builder AS chef
WORKDIR /app

# Build the service
FROM chef AS builder
COPY .sqlx/ /app/.sqlx/
COPY common/ /app/common/
COPY schemas/ /app/schemas/
COPY accounts-backend/ /app/accounts-backend/
RUN cargo build --bin accounts-backend

# Run the binary from a the runtime image
FROM twote-rust-runtime

COPY --from=builder /app/target/debug/accounts-backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/accounts-backend"]
```

Also, modify the [twote-rust-builder](../images/twote-rust-builder/Dockerfile) Docker image to include the `accounts-backend` service:

```Dockerfile
# Copy over the Cargo.toml files of every crate in the workspace
COPY Cargo.toml /app/Cargo.toml
COPY common/Cargo.toml /app/common/Cargo.toml
COPY schemas/Cargo.toml /app/schemas/Cargo.toml
COPY twote-api/Cargo.toml /app/twote-api/Cargo.toml
# Add the new service here:
COPY accounts-backend/Cargo.toml /app/accounts-backend/Cargo.toml
```

## Docker Compose

Next, we need to add the service to [docker-compose.yaml](../docker-compose.yaml) so that it can be run with the rest of the services.

```yaml
  accounts-backend:
    build:
      context: .
      dockerfile: accounts-backend/Dockerfile
    ports:
      - "8082:8082"
    healthcheck:
      test: ["CMD", "grpc-health-probe", "-addr=localhost:8082"]
      interval: 300ms
      timeout: 500ms
      retries: 25
```

The healthcheck is used to ensure that the service is ready to accept requests, helping coordinate the startup of other services.

Once the service is added to `docker-compose.yaml`, we can run it with the rest of the services:

```bash
docker-compose up
```

## Next steps

Now that your Rust + gRPC service is running, you can start adding endpoints.

See https://github.com/hyperium/tonic/tree/master/examples for examples of how to add endpoints to your service.
