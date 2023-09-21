FROM node:16
#FROM node:20-bookworm

WORKDIR /app

COPY . .

# Install wget to download protoc-gen-grpc-web
RUN apt update && apt install -y wget \
    && wget -qO/usr/local/bin/protoc-gen-grpc-web https://github.com/grpc/grpc-web/releases/download/1.4.2/protoc-gen-grpc-web-1.4.2-linux-x86_64 \
    && chmod +x /usr/local/bin/protoc-gen-grpc-web

# Install protoc
RUN apt update && apt install -y protobuf-compiler

# Provide execute permissions for the script
RUN chmod +x ./scripts/proto.sh

# Run the proto script
RUN ./scripts/proto.sh

WORKDIR twote-frontend

# Install npm
RUN npm install

EXPOSE 3000

CMD ["npm", "start"]
