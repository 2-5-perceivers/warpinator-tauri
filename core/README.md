# Warpinator Lib

A simple implementation of the [Warpinator](https://github.com/linuxmint/warpinator) protocol in Rust.
Note that this library is still in development and may not be fully functional yet and does not support the version 1 of
the protocol(as it's considered deprecated and the official Warpinator app has a PR to remove it).

Take a look at the [examples](./examples) folder for a simple example of how to use the library and a simple TUI client
implementation.

## Features

- **event-driven API**
- **stateless by design** — no hidden persistence
- **asynchronous** — built on top of tokio
- **superfast** — optimized for performance and low latency

## TODO

- [x] Implement certificate generation and management
- [x] Implement authentication
- [x] Implement remote management
- [x] Implement message sending
- [x] Implement file transfers
- [ ] Compression
- [ ] ipv6 support