cargo-screeps
=============

[![crates.io version badge][cratesio-badge]][crate]

Build tool for deploying Rust WASM code to Screeps game servers.

Best used with [`screeps-game-api`].

This implements type-safe WASM bindings to the Screeps in-game API.

This is not fully tested, but feel free to use! Issues are welcome.

---

`cargo-screeps` build options:

### `build`:

1. during build stage, shell out to https://github.com/koute/cargo-web for actual building the rust code.
2. strip off header / surrounding function `cargo-web` generates for a generic JS file, since we know we're deploying to node
3. append call to `__initialize` function which cargo-web generates, using `require('compiled')` to get the WASM bytes
4. create `target/main.js` containing processed JS and copy WASM file to `target/compiled.wasm`

### `upload`:

1. run build.
2. read `target/*.js` and `target/*.wasm`, keeping track of filenames
3. read `screeps.toml` for upload options
4. upload all read files to server, using filenames as the filenames on the server.

### `copy`:

1. run build.
2. copy `target/main.js` and `target/compiled.wasm` to `<destination directory>/<branch name>/`
3. if pruning is enabled, delete all other files in `<destination directory>/<branch name>/`

### `deploy`:

1. run build.
2. run `upload` or `copy` depending on the `default_deploy_mode` configuration option.

### `check`:

1. perform type checking / lifetime checking without compiling code
  - runs `cargo web --check --target=wasm32-unknown-unknown` which is fairly similar to
    `cargo check`.

[cratesio-badge]: http://meritbadge.herokuapp.com/cargo-screeps
[crate]: https://crates.io/crates/cargo-screeps/
[`screeps-game-api`]: https://github.com/daboross/screeps-in-rust-via-wasm/

