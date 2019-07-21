cargo-screeps
=============

[![crates.io version badge][cratesio-badge]][crate]

Build tool for deploying Rust WASM code to Screeps game servers.

`cargo-screeps` is a direct wrapper of [`cargo-web`], and depends on it internally. It adds the
ability to trim output files for use in `screeps`, and upload to screeps server.

Intended to be used with [`screeps-game-api`], type-safe WASM bindings to the Screeps in-game API.

---

# Build Options

### `build`:

Configured in `[build]` config section. No required settings.

1. runs `cargo-web build --release` to build the rust source
2. strips off header `cargo-web` generates for loading WASM file from a URL or the local filesystem
3. appends initialization call using bytes from `require('<compiled module name>')`
4. puts processed JS into `target/main.js` copy compiled WASM into `target/compiled.wasm`

### `upload`:

Requires `[upload]` config section with at minimum username, password and branch.

1. runs build
2. reads `target/*.js` and `target/*.wasm`, keeping track of filenames
3. reads `screeps.toml` for upload options
4. uploads all read files to server, using filenames as the filenames on the server

### `copy`:

Requires `[copy]` config section with at minimum destination and branch.

1. runs build
2. copies compiled main file and WASM file (default `main.js` and `compiled.wasm`) from `target/` to
   `<destination directory>/<branch name>/`
3. if pruning is enabled, deletes all other files in `<destination directory>/<branch name>/`

### `deploy`:

Requires `default_deploy_mode` configuration setting.

1. runs build
2. runs `upload` or `copy` depending on the `default_deploy_mode` configuration option

### `check`:

Does not require configuration.

1. performs type checking and lifetime checking without compiling code
  - runs `cargo web check` (see `cargo check` for non-WASM codebases)

# Configuration Options

## No namespace

- `default_deploy_mode`: controls what `cargo screeps deploy` does

  This configuration is required for `cargo screeps deploy`. Possible values are `"copy"`
  and `"upload"`.

## `[upload]`

Options for the `upload` deploy mode.

This section is required to use `cargo screeps upload`.

- `auth_token`: an auth token for your Screeps account
- `username`: your Screeps username or email
- `password`: your Screeps password

  Either an auth_token or your username/password can be supplied. When both are set the auth token is used. For private servers set a password using [screepsmod-auth].
- `branch`: the branch on the server to upload files to
- `ptr`: if true, upload to the "ptr" realm
- `hostname`: the hostname to upload to

  For example, this could be `screeps.com`, `localhost` or `server1.screepsplu.us`.
- `ssl`: whether to connect to the server using ssl

  This should generally be true for the main server and false for private servers.
- `port`: port to connect to server with

  This should generally be set to `21025` for private servers.

## `[copy]`

Options for the `copy` deploy mode.

This section is required to use `cargo screeps copy`.

- `destination`: the directory to copy files into

  If this path is not absolute, it is interpreted as relative to `screeps.toml`
- `branch`: the "branch" to copy into

  This is the subdirectory of `destination` which the js/wasm files will be copied into.
- `prune`: if true, extra files found in the destination/branch directory will be deleted

## `[build]`

This configures general build options.

- `output_js_file`: the javascript file to export bindings and bootstrapping as
  (default `"main.js"`)
- `output_wasm_file`: the WASM file to rename compile WASM to (default `"compiled.wasm"`)
- `initialize_header_file`: a file containing the JavaScript for starting the WASM instance. See
  [overriding the default initialization header](#overriding-the-default-initialization-header)

## Overriding the default initialization header

`cargo-screeps` tries to make a reasonable `main.js` file to load the WASM. However, it's pretty
basic, and you might find you want to do some things in JavaScript before loading the WASM module.

Luckily, you can override this initialization! Set `build.initialize_header_file` to a file
containing the JavaScript initialization code.

Two utility functions `wasm_fetch_module_bytes` and `wasm_create_stdweb_vars` will always be
created, but the initialization header controls what actually runs.

See [docs/initialization-header.md] for more information on this.

# Updating `cargo screeps`

To update `cargo-screeps`, simply repeat the install process with the `--force` (`-f`) flag.

After updating, you'll want to do a full `cargo clean` to remove any old artifacts which were built
using the older version of `cargo-screeps`.

```sh
cargo install -f cargo-screeps
cargo clean
cargo screeps build
```

[cratesio-badge]: http://meritbadge.herokuapp.com/cargo-screeps
[crate]: https://crates.io/crates/cargo-screeps/
[`screeps-game-api`]: https://github.com/daboross/screeps-in-rust-via-wasm/
[`cargo-web`]: https://github.com/koute/cargo-web
[screepsmod-auth]: https://www.npmjs.com/package/screepsmod-auth

