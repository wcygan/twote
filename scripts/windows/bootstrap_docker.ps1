docker-compose down

docker pull lukemathwalker/cargo-chef:latest-rust-1.72

docker pull node:16

docker build -f images/twote-javascript-runtime/Dockerfile -t twote-javascript-runtime .

docker build -f images/twote-rust-builder/Dockerfile -t twote-rust-builder .

docker build -f images/twote-rust-runtime/Dockerfile -t twote-rust-runtime .

docker-compose build

docker-compose up -d