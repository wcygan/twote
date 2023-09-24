FROM debian:bookworm
WORKDIR /app

# Install wget to download grpc-health-probe
RUN apt update && apt install -y wget \
    && wget -qO/usr/local/bin/grpc-health-probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/v0.4.5/grpc_health_probe-linux-amd64 \
    && chmod +x /usr/local/bin/grpc-health-probe