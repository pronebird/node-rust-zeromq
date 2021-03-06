# Interprocess communication between Node.js client and Rust server using Protocol Buffers and ZeroMQ

## Requirements

1. Node.js
2. Rust compiler

## Dependencies

1. Switch to `client` and run `npm install`
2. Switch to `server` and run `cargo update`

## Protocol buffers interface

Run `gen-protocol.sh` to update protocol buffers interface in Rust to reflect changes to .proto files in protocol folder. Node.js implementation does that automatically in runtime.

## Running client

```sh
cd client
node client.js
```

## Running server

```sh
cd server
cargo run
```
