cargo-screeps
=============

[![crates.io version badge][cratesio-badge]][crate]

Build tool for deploying Rust WASM code to Screeps game servers.

Best used with [`screeps-game-api`].

This implements type-safe WASM bindings to the Screeps in-game API.

This is not fully tested, but feel free to use! Issues are welcome.

---

# Build Options

### `build`:

Configured in `[build]` config section. No required settings.

1. run https://github.com/koute/cargo-web to actually build rust source
2. strip off header / surrounding function `cargo-web` generates for a generic JS file to load from
   web or from local filesystem
3. append initialization call using bytes from `require('<compiled module name>')`
4. put processed JS into `target/main.js` copy compiled WASM into `target/compiled.wasm`

### `upload`:

Requires `[upload]` config section with at minimum username, password and branch.

1. run build
2. read `target/*.js` and `target/*.wasm`, keeping track of filenames
3. read `screeps.toml` for upload options
4. upload all read files to server, using filenames as the filenames on the server

### `copy`:

Requires `[copy]` config section with at minimum destination and branch.

1. run build
2. copy compiled main file and WASM file (default `main.js` and `compiled.wasm`) from `target/` to
   `<destination directory>/<branch name>/`
3. if pruning is enabled, delete all other files in `<destination directory>/<branch name>/`

### `deploy`:

Requires `default_deploy_mode` configuration setting.

1. run build
2. run `upload` or `copy` depending on the `default_deploy_mode` configuration option

### `check`:

Does not require configuration.

1. perform type checking / lifetime checking without compiling code
  - runs `cargo web --check --target=wasm32-unknown-unknown` which is fairly similar to
    `cargo check`.

# Configuration Options

## No namespace

- `default_deploy_mode`: controls what `cargo screeps deploy` does

  This configuration is required for `cargo screeps deploy`. Possible values are `"copy"`
  and `"upload"`.

## `[upload]`

Options for the `upload` deploy mode.

This section is required to use `cargo screeps upload`.

- `username`: your Screeps username or email
- `password`: your Screeps password

  For private servers set a password using [screepsmod-auth].
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

Luckily, you can override this initialization! Set `build.initialize_header_file` to a file containing the JavaScript initialization code.

The code will have access to two utility functions, `wasm_fetch_module_bytes` and `wasm_create_stdweb_vars`:

```js
/**
 * Fetches WASM bytes for the wasm module.
 *
 * These should be given to `new WebAssembly.Module` to instantiate a WebAssembly module.
 */
function wasm_fetch_module_bytes() { /* ... */ }

/**
 * Creates the stdweb wrapper for a module instance.
 *
 * It has two properties, `imports` and `initialize`. `imports` should be passed as the second
 * argument to `new WebAssembly.Instance` to provide the WASM module with imports, and `initialize`
 * should be called on the resulting WebAssembly instance.
 *
 * Calling `initialize` will finish associating the imports with the wasm module, and will call the
 * rust module's main function.
 */
function wasm_create_stdweb_vars() { /* ... */ }
```

When you override the initialization, you will need to perform it yourself. Here is the default initialzation header for reference:

```js
"use strict";
let wasm_module = null;

function wasm_initialize() {
    if (wasm_module == null) {
        let wasm_bytes = wasm_fetch_module_bytes();
        wasm_module = new WebAssembly.Module(wasm_bytes);
    }
    let stdweb_vars = wasm_create_stdweb_vars();
    let wasm_instance = new WebAssembly.Instance(wasm_module, stdweb_vars.imports);
    stdweb_vars.initialize(wasm_instance);
    // assume the WASM main overrides this
    module.exports.loop();
}

module.exports.loop = wasm_initialize;
```

# Updating `cargo screeps`

As it parses the unstable output of `cargo-web`, `cargo-screeps` is highly dependent on `cargo-web`
version. It is recommended to upgrade both together.

Installing a version of `cargo-web` newer than what `cargo-screeps` supports will cause it to
output an error on build. If this happens, please create an issue on this repository and we can
update `cargo-screeps`. Updating it is simple, but it needs to be done every time `cargo-web`
changes the output format, and we might not realize that has happened.

After updating, you'll want to do a full `cargo clean` to remove any old artifacts which were built
using the older version of `cargo-web`.

```sh
cargo install -f cargo-web
cargo install -f cargo-screeps
cargo clean
cargo screeps build
```

[cratesio-badge]: http://meritbadge.herokuapp.com/cargo-screeps
[crate]: https://crates.io/crates/cargo-screeps/
[`screeps-game-api`]: https://github.com/daboross/screeps-in-rust-via-wasm/
[screepsmod-auth]: https://www.npmjs.com/package/screepsmod-auth

