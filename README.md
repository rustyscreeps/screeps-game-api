screeps-game-api
================

[![Linux Build Status][travis-image]][travis-builds]
[![crates.io version badge][cratesio-badge]][crate]
[![docs.rs version badge][docsrs-badge]][docs]

![Rusty Screeps Logo][logo]

Typed bindings to the Screeps in-game API for WASM Rust AIs.

Also the homepage for tools relating to writing [Screeps] AIs in Rust.

`screeps-game-api` is a Rust binding to the JavaScript APIs for programs compiled to WASM using
[`stdweb`].

Also in this organization is [`cargo screeps`][cargo-screeps], a binary program which wraps `cargo
web` and lets one directly upload Rust WASM code to Screeps servers.

These two tools go together well, but do not depend on eachother. `cargo-screeps` can compile and
upload any screeps WASM project buildable with `stdweb`'s `cargo-web`, and `screeps-game-api` is
usable in any project built with `cargo-web`.

Writing Screeps code in Rust can be nice, but it can also be annoying. If you have tips, tricks, or
other things you'd like to share, make an issue! We need to write more documentation, and if we have
enough ideas, we can start an mdbook for this repository.

- [cargo screeps usage docs][cargo-screeps]
- [screeps-game-api api docs][docs]
- [screeps-starter-rust example project](https://github.com/rustyscreeps/screeps-starter-rust/)

If you need help or want to share feedback, feel free to open an
[issue](https://github.com/rustyscreeps/screeps-game-api/issues)
or come say "_Hi!_" on [the official Screeps Slack](https://chat.screeps.com/) in the `#rust-wasm`
channel!

---

Here's a quickstart for what you *need* to get going. More documentation will be made in the future.

```sh
# clone:

git clone https://github.com/rustyscreeps/screeps-starter-rust.git
cd screeps-starter-rust

# cli dependencies:

cargo install cargo-screeps

# configure for uploading:

cp example-screeps.toml screeps.toml
nano screeps.toml

# build tool:

cargo screeps --help
```

[screeps]: https://screeps.com/
[`stdweb`]: https://github.com/koute/stdweb
[travis-image]: https://travis-ci.org/rustyscreeps/screeps-game-api.svg?branch=master
[travis-builds]: https://travis-ci.org/rustyscreeps/screeps-game-api
[docsrs-badge]: https://docs.rs/screeps-game-api/badge.svg
[cratesio-badge]: http://meritbadge.herokuapp.com/screeps-game-api
[docs]: https://docs.rs/screeps-game-api/
[crate]: https://crates.io/crates/screeps-game-api/
[cargo-screeps]: https://github.com/rustyscreeps/cargo-screeps/
[logo]: ./logo.png
