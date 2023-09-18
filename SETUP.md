Follow these instructions to set up the project on your local machine. You will install dependencies & compile `.proto` files.

## 1: Install protocol buffers

Ensure you have `protoc` installed:
1. https://grpc.io/docs/protoc-installation/

Ensure you have `protoc-gen-grpc-web` installed:
1. https://github.com/grpc/grpc-web#code-generator-plugin
2. OR https://formulae.brew.sh/formula/protoc-gen-grpc-web

## 2: Install the Rust toolchain

Follow https://www.rust-lang.org/tools/install to install the Rust toolchain.

This will install `rustc` (the Rust compiler), `cargo` (the Rust package manager), and `rustup` (the Rust toolchain manager).

## 3: Install npm (a Javascript package manager)

Follow https://docs.npmjs.com/downloading-and-installing-node-js-and-npm to install `npm`.

After installing `npm`, run `npm install`. This will install of the project's Javascript dependencies.

## 4: Install Envoy (a proxy server)

Follow https://www.envoyproxy.io/docs/envoy/latest/start/install and ensure you have `envoy` installed.

Envoy is used to proxy the gRPC-Web requests to the gRPC server.

## 5: compile the proto files

Generate the Rust & Javascript code from the proto files:

```
chmod +x ./scripts/proto.sh && ./scripts/proto.sh
```

## 6: Run the server

See [README.md](README.md#running-the-app) for instructions on how to run the server.