screeps-game-api
================

[![Linux Build Status][actions-badge]][actions-builds]
[![crates.io version badge][cratesio-badge]][crate]
[![dependency status][deps-badge]][deps]
[![docs.rs version badge][docsrs-badge]][docs]

![Rusty Screeps Logo][logo]

Typed bindings to the Screeps in-game API for WASM Rust AIs.

Also the homepage for tools relating to writing [Screeps] AIs in Rust.

`screeps-game-api` is a Rust binding to the JavaScript APIs for programs compiled to WASM using
[`wasm-pack`].

Also in this organization is [`cargo screeps`][cargo-screeps], a binary program which wraps
`wasm-pack` and lets one directly upload Rust WASM code to Screeps servers.

These two tools go together well, but do not depend on eachother. `cargo-screeps` can compile and
upload any screeps WASM project buildable with `wasm-bindgen`'s `wasm-pack`, and `screeps-game-api` is
usable in any project built with `wasm-pack`.

Writing Screeps code in Rust can be nice, but it can also be annoying. If you have tips, tricks, or
other things you'd like to share, make an issue! We need to write more documentation, and if we have
enough ideas, we can start an mdbook for this repository.

- [cargo screeps usage docs][cargo-screeps]
- [screeps-game-api api docs][docs]
- [screeps-starter-rust example project](https://github.com/rustyscreeps/screeps-starter-rust/)

If you need help or want to share feedback, feel free to open an
[issue](https://github.com/rustyscreeps/screeps-game-api/issues)
or come say "_Hi!_" on [the official Screeps Discord](https://discord.gg/screeps) in the `#rust`
channel!

---

Here's a quickstart for what you *need* to get going. More documentation will be made in the future.

```sh
# Install CLI dependency:
cargo install cargo-screeps

# Clone the starter
git clone https://github.com/rustyscreeps/screeps-starter-rust.git
cd screeps-starter-rust

# Copy the example config, and set up at least one deployment mode
cp example-screeps.toml screeps.toml
nano screeps.toml
# configure credentials (API key) if you'd like to upload directly,
# or a directory to copy to if you'd prefer to use the game client to deploy

# build tool:
cargo screeps --help
# compile the module without deploying anywhere
cargo screeps build
# compile plus deploy to the configured 'upload' mode; any section name you
# set up in your screeps.toml for different environments and servers can be used
cargo screeps deploy -m upload
# or if you've set a default mode in your configuration, simply use:
cargo screeps deploy
```

[screeps]: https://screeps.com/
[`wasm-pack`]: https://rustwasm.github.io/wasm-pack/
[actions-badge]: https://github.com/rustyscreeps/screeps-game-api/actions/workflows/build.yml/badge.svg
[actions-builds]: https://github.com/rustyscreeps/screeps-game-api/actions/workflows/build.yml
[cratesio-badge]: https://img.shields.io/crates/v/screeps-game-api.svg
[crate]: https://crates.io/crates/screeps-game-api/
[deps-badge]: https://deps.rs/repo/github/rustyscreeps/screeps-game-api/status.svg
[deps]: https://deps.rs/repo/github/rustyscreeps/screeps-game-api
[docsrs-badge]: https://docs.rs/screeps-game-api/badge.svg
[docs]: https://docs.rs/screeps-game-api/
[cargo-screeps]: https://github.com/rustyscreeps/cargo-screeps/
[logo]: ./logo.png
