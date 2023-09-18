# Twote

[One Pager](https://docs.google.com/document/d/14h-WVhfJx1pfHTL0zWkMsaf66OjMi0LC3AF4IJZPIiI/edit)

A social media app where users can post messages and follow other users.

## Prerequisites

You will need to install:
1. [protoc](https://grpc.io/docs/protoc-installation/)
    - protoc is used to generate the gRPC code.
2. [protoc-gen-grpc-web](https://github.com/grpc/grpc-web/releases)
    - protoc-gen-grpc-web is used to generate the gRPC-Web code.
3. [Rust](https://www.rust-lang.org/tools/install)
    - The Rust toolchain is used to compile Rust code.
4. [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
    - npm is used to install the Javascript dependencies.
5. [envoy](https://www.envoyproxy.io/docs/envoy/latest/start/install)
    - Envoy is used to proxy the gRPC-Web requests to the gRPC server.

Generate the Rust & Javascript code from the proto files:

```
chmod +x ./scripts/proto.sh && ./scripts/proto.sh
```

## Running the app

Run all of these commands in the root directory in separate terminals.

Start the gRPC server:

```
cargo run --bin twote-api
```

Start the Envoy proxy:

```
envoy -c envoy.yaml
```

Start the Javascript server:

```
cd twote-frontend && npm start
```

Finally, open http://localhost:3000/ in your browser.