# CHOAM

> **CHOAM**: The Chaum-Pedersen Heighliner Orbital Authentication Machine.

*tl;dr* - an authentication server that uses a custom chaum-pedersen protocol implementation to authenticate requests and hand out JWTs for authorization.

I've been reading a lot of Dune lately, so I present to you *CHOAM*, a Chaum-Pedersen protocol implementation in GRPC and Rust. Chaum-Pedersen is a Sigma protocol for [zero-knowledge proofs](https://en.wikipedia.org/wiki/Zero-knowledge_proof).

![CHOAM In Action](screenshot.png)

## I am not a cryptographer, and I'm certainly not *your* cryptographer

This is not production ready code, and it should absolutely not be used for anything in production period.

## Running CHOAM

To test this script out, you need to run the server locally and then run the client to authenticate with the server.

- `cargo run-server` to run the CHOAM server.
- `cargo run-client` to run the client.
- `cargo build-client` builds the client binary.
- `card build-server` builds the server binary.

The client runs an automatic connection protocol and stores the token it receives from the authentication request.

## Structure

- `src/main.rs` contains a heavily commented walk-through of the Chaum-Pedersen protocol.
- `src/server.rs` contains the gRPC server implementation for authenticating against.
- `src/client.rs` contains the gRPC client implementation that authenticates itself against the server.
- `src/proto/` contains the Protobuf definitions.

## Dependencies

- Tokio for asynchronous execution at runtime
- Tonic for generating Protobuf files
- num-bigint for modpow and other math
- jsonwebtoken for generating JWTs after successful authentication

## Related Reading

- [Zero-knowledge proof](https://en.wikipedia.org/wiki/Zero-knowledge_proof)
- [Publicly verifiable secret sharing](https://en.wikipedia.org/wiki/Publicly_Verifiable_Secret_Sharing)

*The spice must flow.*
