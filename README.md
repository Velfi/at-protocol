# at-protocol
My feeble attempt at implementing https://atproto.com

> The **Authenticated Transfer Protocol**, aka **ATP**, is a protocol for large-scale distributed social applications.

See [here][atp-overview] for an overview of the protocol.

## I'm working on:

- [x] Reading the [Lexicon] files specifying the operations for an XRPC service.
- [ ] Creating an intermediate representation of the service client.
  - [x] Create a module tree so that codegen can be directed into specific Rust modules.
- [ ] Generating a Rust crate from the IR.
  - [x] Generate a `Cargo.toml` crate manifest.
  - [ ] Generate `Input`s that can be converted into HTTP requests.
  - [ ] Generate a builder struct for each `Input`.
  - [ ] Generate `Output`s that can be created from HTTP responses.
  - [ ] Generate a builder struct for each `Output`.
- [ ] Sending a request to an XRPC service.
  - [ ] Create a common HTTP client that can be used to send XRPC requests.
  - [ ] Create a Rust implementation of [placeholder DIDs].
  - [ ] Create an XRPC-compatible service for testing.

[atp-overview]: https://atproto.com/guides/overview
[Lexicon]: https://atproto.com/specs/lexicon
[XRPC]: https://atproto.com/specs/xrpc
[placeholder DIDs]: https://atproto.com/specs/did-plc
