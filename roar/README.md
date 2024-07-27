# Roar

[Challenge Url](https://codingchallenges.fyi/challenges/challenge-redis)

A Redis server clone written in Rust, with full support for Redis's RESP protocol.
Supportes the follwing commands:

- PING
- ECHO
- SET
- GET
- INCR
- DECR
- EXISTS
- DEL
- LPUSH
- RPUSH

## How to run

To start the server:

```shell
cargo run
```

Then you can start sending requests to the server using `redis-cli`:

```shell
redis-cli ping
```

> [!NOTE]  
> You get `redis-cli` installed when you [install redis](https://redis.io/docs/latest/operate/oss_and_stack/install/install-redis/).
