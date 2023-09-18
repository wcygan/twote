# Generate the javascript code from the proto files
# Run this script from the root of the project
protoc \
    --proto_path=schemas/protos/ \
    --js_out=import_style=commonjs,binary:schemas/js \
    --grpc-web_out=import_style=commonjs,mode=grpcwebtext:schemas/js \
    schemas/protos/hello.proto