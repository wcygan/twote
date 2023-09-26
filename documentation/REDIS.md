# Redis

Redis is a key-value store used for caching. Twote uses it to cache authorization tokens.

## Running Redis

You can run a Redis server with the following command:

```bash
docker-compose up -d token-cache
```

## Connecting to Redis via CLI

You can connect to the Redis server via CLI with the following command:

```bash
docker exec -it twote_token-cache_1 redis-cli
```

Then you can issue commands like `get`, `set`, and `del`:

```bash
set foo bar
get foo
del foo
```