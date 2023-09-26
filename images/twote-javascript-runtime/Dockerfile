FROM node:16

WORKDIR /app

# Install wget to download protoc-gen-grpc-web
RUN apt update && apt install -y wget \
&& wget -qO/usr/local/bin/protoc-gen-grpc-web https://github.com/grpc/grpc-web/releases/download/1.4.2/protoc-gen-grpc-web-1.4.2-linux-x86_64 \
    && chmod +x /usr/local/bin/protoc-gen-grpc-web

# Install protoc
RUN apt update && apt install -y protobuf-compiler

# Set working directory
WORKDIR /app

# Copy package.json and package-lock.json for caching
COPY twote-web/package*.json ./

# Install dependencies
RUN npm install