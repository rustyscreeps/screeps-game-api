Screeps in Rust (via WASM)
==========================

Tools for creating [Screeps] AIs written in Rust.


`screeps-game-api` is a Rust binding to the JavaScript APIs for programs compiled to WASM using [`stdweb`].

`cargo screeps` is a binary program which wraps `cargo web` and lets one directly upload Rust WASM code to the
Screeps server.

These two tools go together well, but do not depend on eachother. `cargo-screeps` can compile and upload
any screeps WASM project buildable with `stdweb`'s `cargo-web`, and `screeps-game-api` is usable in any
project built with `cargo-web`.

See https://github.com/daboross/screeps-starter-rust/ for an example AI using these libraries.
[#3#](https://github.com/daboross/ai-3) is another example, but it is not a minimal AI.

Here's a quickstart guide. More documentation will be made in the future.

```sh
# clone:

git clone https://github.com/daboross/screeps-starter-rust.git
cd screeps-starter-rust

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
