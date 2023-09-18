# Twote

[One Pager](https://docs.google.com/document/d/14h-WVhfJx1pfHTL0zWkMsaf66OjMi0LC3AF4IJZPIiI/edit)

A social media app where users can post messages and follow other users.


## Running the app

See [SETUP.md](SETUP.md) for instructions on how to set up the project on your local machine.

Run these commands in the root directory in separate terminals.

### Start the server

```
cargo run --bin twote-api
```

### Start the proxy

```
envoy -c envoy.yaml
```

### Start the client:

```
cd twote-frontend && npm start
```

Next, open http://localhost:3000/ in your browser.