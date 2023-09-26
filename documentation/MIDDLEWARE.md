# Middleware

Middleware is how we hook into the request/response cycle of gRPC. 

It can be used to add logging, authentication, and more. Further, it can be used either before a request is sent to the server, or after a response is received from the server.
## Backend Middleware

`twote-api` uses authentication middleware, [AuthMiddleware](../common/src/middleware/authentication.rs), to ensure that only authenticated users can access certain endpoints.

`twote-api` exposes two endpoints which do not require authentication: `CreateAccount` and `Login`. All other endpoints require authentication.

The authentication middleware checks if the request has an authorization token, and if it does, it checks redis to see if the token is valid. If the token is valid, the request is passed to the next middleware. If the token is invalid, the request is rejected. 

See PRs [#14](https://github.com/wcygan/twote/pull/14) & [#15](https://github.com/wcygan/twote/pull/15) for how backend authentication

## Frontend Middleware

`twote-web` uses middleware to add authentication tokens to requests & to invalidate sessions for unauthenticated users.

See PRs [#15](https://github.com/wcygan/twote/pull/15) & [#16](https://github.com/wcygan/twote/pull/16) for how frontend authentication middleware was added.