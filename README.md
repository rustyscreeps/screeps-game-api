Screeps in Rust (via WASM)
==========================

Tools for creating [Screeps] AIs written in Rust.


`screeps-game-api` is a Rust binding to the JavaScript APIs for programs compiled to WASM using [`stdweb`].

`cargo screeps` is a binary program which wraps `cargo web` and lets one directly upload Rust WASM code to the
Screeps server.

See https://github.com/daboross/screeps-starter-python/ for an example AI using these libraries.
[#3#](https://github.com/daboross/p3p) is another example, but it is not a minimal AI.

Here's a quickstart guide. More documentation will be made in the future.

```sh
# clone:

git clone https://github.com/daboross/screeps-starter-rust.git
cd screeps-starter-rust
rustup override set nightly

# cli dependencies:

cargo install cargo-screeps
cargo install cargo-web

# configure for uploading:

cp example-screeps.toml screeps.toml
nano screeps.toml

# build tool:

cargo screeps --help
```

[screeps]: https://screeps.com/
[`stdweb`]: https://github.com/koute/stdweb
