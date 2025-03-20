## Description

This is a simple clone of the Redis database written in Rust that supports basic commands like `SET`, `GET`, `ECHO`, and `PING` using the standard Redis encoding format. The application can be interacted with using the `redis-cli` client and follows the standard Redis protocol.<br>
![My Skills](https://skillicons.dev/icons?i=rust)

## Features

The application supports the following Redis commands:

- `SET <key> <value> [PX <seconds>]`: Sets the value for the specified key. You can optionally set an expiration time for the key with the `PX` option, which specifies the number of seconds after which the key will expire.
- `GET <key>`: Retrieves the value associated with the given key.
- `ECHO <message>`: Returns the provided message.
- `PING`: Returns a `PONG` response to check if the server is running.

## Installation

To run this simple Redis clone, follow the instructions below:

1. Clone repository:

   ```bash
   git clone https://github.com/PatrysUwU/Redis_rust/
   cd simple-redis-clone
   ```
2. Build the project:
    ```bash
    cargo build --release
    ```
3. Run the application:
    ```bash
    cargo run
    ```

# Usage

1. If you don't have redis cli installed download it from [Redis website](https://redis.io/download)

2. Open another terminal and type:
    ```bash
    redis-cli
    ```
3. Now you can use commands listed above

# Supported protocol
The application communicates using the standard Redis RESP (REdis Serialization Protocol) format. It can handle simple text-based commands and replies, as expected from Redis clients like redis-cli.