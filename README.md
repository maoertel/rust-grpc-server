# gRPC with tonic written in Rust

Small application that spins up a gRPC server to play around with [tonic](https://github.com/hyperium/tonic).

1. Start the server with `cargo run`.
2. Install an `grpc` application for commandline (on mac e.g. `brew install grpc`).
3. Ask for a movie: `grpc_cli call localhost:50051 moviestore.Moviestore.GetMovie "id: '73'"`.

The output should look as follows:

```shell
connecting to localhost:50051
Received initial metadata from server:
date : ...
id: "73"
title: "Lord of the Rings"
director: "Peter Jackson"
year: 2000
Rpc succeeded with OK status
```

