Screeps in Rust (via WASM)
==========================

Tools for creating [Screeps] AIs written in Rust.


`screeps-game-api` is a Rust binding to the JavaScript APIs for programs compiled to WASM using [`stdweb`].

`cargo screeps` is a binary program which wraps `cargo web` and lets one directly upload Rust WASM code to the
Screeps server.

These two tools go together well, but do not depend on eachother. `cargo-screeps` can compile and upload
any screeps WASM project buildable with `stdweb`'s `cargo-web`, and `screeps-game-api` is usable in any
project built with `cargo-web`.

Writing Screeps code in Rust can be nice, but it can also be annoying. If you have tips, tricks, or other
things you'd like to share, make an issue! We need to write more documentation, and if we have some ideas
of things to include, we can start an mdbook in this repository.

---

See https://github.com/daboross/screeps-starter-rust/ for a small example AI using these libraries.

Here's a quickstart for what you *need* to get going. More documentation will be made in the future.

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

One more note about updating `cargo-screeps`:

`cargo-screeps` is highly dependent on the version of `cargo-web`, so updating both at the same time is usually recommended. Some `cargo-web` versions might be released and break the `cargo-screeps` interface. In this case, building will fail and output a message about creating an issue on this repository.

After I've updated `cargo-screeps` to work with the new `cargo-web`, you'll want to update both, then run `cargo clean` to clean out your old target directory. After all of this, you should be good to go!

```sh
cargo install -f cargo-web
cargo install -f cargo-screeps
cargo clean
cargo screeps --build
```

[screeps]: https://screeps.com/
[`stdweb`]: https://github.com/koute/stdweb
