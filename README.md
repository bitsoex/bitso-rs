[![Crates.io](https://img.shields.io/crates/v/bitsors.svg)](https://crates.io/crates/bitsors)
[![Docs](https://docs.rs/bitsors/badge.svg)](https://docs.rs/bitsors)
[![CI checks](https://github.com/bitsoex/bitso-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/bitsoex/bitso-rs/actions/workflows/ci.yml)
[![rustc](https://img.shields.io/badge/rust-1.56%2B-orange.svg)](https://img.shields.io/badge/rust-1.56%2B-orange.svg)

# bitsors
A Rust wrapper for the [Bitso API](https://bitso.com/api_info/), with support for the [public](https://bitso.com/api_info#public-rest-api) and [private](https://bitso.com/api_info#private-rest-api) API endpoints, plus the [WebSocket API](https://bitso.com/api_info#websocket-api).



## Testing

```bash
cargo test
```

We also make sure Clippy is happy with us with: 

```bash
cargo clippy --all-features --all-targets -- -D warnings
```

## Examples

You can find lots of examples under the `examples/` folder!

## MSRV

`1.56.1`
