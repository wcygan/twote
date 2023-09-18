# Generate the javascript code from the proto files
# Run this script from the root of the project
protoc \
    --proto_path=schemas/protos/ \
    --js_out=import_style=commonjs,binary:twote-frontend/src/proto \
    --grpc-web_out=import_style=commonjs,mode=grpcwebtext:twote-frontend/src/proto \
    schemas/protos/*.proto

cargo build