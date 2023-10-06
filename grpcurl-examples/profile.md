[grpcurl](https://github.com/fullstorydev/grpcurl) examples for [profile.proto](../schemas/protos/profile.proto)

## Create

Request:
```bash
grpcurl -proto schemas/protos/profile.proto -plaintext -d '{"user_id": "a", "first_name": "b", "last_name": "c"}' localhost:8083 profile.ProfileService/Create
```

Response:
```json
{}
```

## Get

Request:

```
grpcurl -proto schemas/protos/profile.proto -plaintext -d '{"user_id": "a"}' localhost:8083 profile.ProfileService/Get
```

Response:
```json
{
  "userId": "a",
  "firstName": "b",
  "lastName": "c",
  "joinedAt": "2023-10-06T05:02:43Z"
}
```

## BatchGet

Request:

```
grpcurl -proto schemas/protos/profile.proto -plaintext -d '{"user_ids": ["a", "b"]}' localhost:8083 profile.ProfileService/BatchGet
```

Response:
```json
{
  "profiles": [
    {
      "userId": "a",
      "firstName": "b",
      "lastName": "c",
      "joinedAt": "2023-10-06T05:02:43Z"
    }
  ]
}
```