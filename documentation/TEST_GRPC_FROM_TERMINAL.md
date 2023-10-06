# Testing gRPC from the terminal 

Make sure your application is running

```bash
./scripts/bootstrap_docker.sh
```

You may need to setup data in your databases before testing the gRPC endpoints. You can do this by opening http://localhost:3000/, creating an account, logging in, and doing any actions that you need to do, such as posting a tweet.

# Testing gRPC from the terminal

You can run a command such as the following to test gRPC endpoints from the terminal.

```bash
grpcurl -proto schemas/protos/profile.proto -plaintext -d '{"user_id": "611f56d0-3f3f-4729-be91-df2004b50800"}' localhost:8083 profile.ProfileService/Get
```

This command builds a request to the `Get` endpoint of the `ProfileService` service. The request body is the JSON object `{"user_id": "611f56d0-3f3f-4729-be91-df2004b50800"}`. The `-proto` flag specifies the path to the `.proto` file that defines the protobuf schema.