Screeps in Rust (via WASM)
==========================

Tools for creating [Screeps] AIs written in Rust.


`screeps-game-api` is a Rust binding to the JavaScript APIs for programs compiled to WASM using [`stdweb`].

`cargo screeps` is a binary program which wraps `cargo web` and lets one directly upload Rust WASM code to the
Screeps server.

See https://github.com/daboross/p3p for an example screeps AI using these libraries. It's not a *minimal* example at all,
but feel free to copy the configuration (Cargo.toml, config-default.toml) into your own project to start.

Here's a minimal quickstart guide. More documentation will be made in the future.

```sh
# clone project:

git clone https://github.com/daboross/p3p.git
cd p3p
rustup override set nightly

# install cli dependencies:

cargo install cargo-screeps
cargo install cargo-web

# build:

cargo screeps --help
```

[screeps](https://screeps.com/)
[`stdweb`](https://github.com/koute/stdweb)
