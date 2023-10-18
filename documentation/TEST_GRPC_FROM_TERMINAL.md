# Testing gRPC from the terminal 

Make sure your application is running

```bash
./scripts/bootstrap_docker.sh
```

You may need to setup data in your databases before testing the gRPC endpoints. You can do this by opening http://localhost:3000/, creating an account, logging in, and doing any actions that you need to do, such as posting a tweet.

# Testing gRPC from the terminal

You will need [grpcurl](https://github.com/fullstorydev/grpcurl) installed.

Next, find example requests in the [grpcurl-examples](../grpcurl-examples) folder.

For example, [profile.md](../grpcurl-examples/profile.md) contains examples for [profile.proto](../schemas/protos/backend/profile.proto).