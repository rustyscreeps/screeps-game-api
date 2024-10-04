screeps-game-api
================

[![Build Status][actions-badge]][actions-builds]
[![crates.io version badge][cratesio-badge]][crate]
[![dependency status][deps-badge]][deps]
[![docs.rs version badge][docsrs-badge]][docs]

![Rusty Screeps Logo][logo]

Typed bindings to the Screeps in-game API for WASM Rust AIs.

Also the homepage for tools relating to writing [Screeps] AIs in Rust.

`screeps-game-api` is a Rust binding to the JavaScript APIs for programs compiled to WASM using
[`wasm-pack`].

Please see the [screeps-starter-rust example project] for an example with instructructions for
getting started, or the [docs] for a detailed API reference.

Writing Screeps code in Rust can be nice, but it can also be annoying. If you have tips, tricks, or
other things you'd like to share, make an issue! We need to write more documentation, and if we have
enough ideas, we can start an mdbook for this repository.

If you need help or want to share feedback, feel free to open an
[issue](https://github.com/rustyscreeps/screeps-game-api/issues)
or come say "_Hi!_" on [the official Screeps Discord](https://discord.gg/screeps) in the `#rust`
channel!

[screeps]: https://screeps.com/
[`wasm-pack`]: https://rustwasm.github.io/wasm-pack/
[screeps-starter-rust example project]: https://github.com/rustyscreeps/screeps-starter-rust/
[actions-badge]: https://github.com/rustyscreeps/screeps-game-api/actions/workflows/check.yml/badge.svg
[actions-builds]: https://github.com/rustyscreeps/screeps-game-api/actions/workflows/check.yml
[cratesio-badge]: https://img.shields.io/crates/v/screeps-game-api.svg
[crate]: https://crates.io/crates/screeps-game-api/
[deps-badge]: https://deps.rs/repo/github/rustyscreeps/screeps-game-api/status.svg
[deps]: https://deps.rs/repo/github/rustyscreeps/screeps-game-api
[docsrs-badge]: https://docs.rs/screeps-game-api/badge.svg
[docs]: https://docs.rs/screeps-game-api/
[logo]: ./logo.png
