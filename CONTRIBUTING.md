# Contributing to `cargo-screeps` and `screeps-game-api`:

1. Ask questions. We're all in the `#rust-wasm` channel on the [screeps slack].
2. Make PRs early, often. We'll review code thoroughly and help where possible.
3. Issues are your guide for what needs to be done. If you think something needs doing and it isn't
   already an issue, make it one!

# Style

We adhere to the following guidelines:

- Code Formatting: [fmt-rfcs Rust Style Guide]
- API building: [Rust API Guidelines]

# Code Formatting

Please run `rustfmt` on the files you edit when submitting to this repository. This will handle all
style issues mentioned in the 'fmt-rfcs' guidelines, so you should be able to simply run `cargo fmt`
and be done with it.

To install `rustfmt`, use `rustup`:

```console
$ rustup component add --toolchain nightly rustfmt-preview
```

Then to format the code in this repository, use the following:

```console
$ cargo fmt
```

[screeps slack]: https://chat.screeps.com/
[fmt-rfcs Rust Style Guide]: https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md
[Rust API Guidelines]: https://rust-lang-nursery.github.io/api-guidelines/
