use std::{fs, process, path::Path};

use {failure, find_folder};

// __initialize defined by stdweb.
// it's signature is 'function __initialize( __wasm_module, __load_asynchronously ) {'
pub static SCREEPS_JS_INITIALIZE_CALL: &str = r#"
__initialize(new WebAssembly.Module(require('compiled')), false);
"#;

pub fn check() -> Result<(), failure::Error> {
    debug!("running check");
    let source_project = find_folder::Search::Parents(2).for_folder("source")?;

    debug!("running 'cargo check --target=wasm32-unknown-unknown'");
    let cargo_success = process::Command::new("cargo")
        .args(&["check", "--target=wasm32-unknown-unknown"])
        .current_dir(&source_project)
        .spawn()?
        .wait()?;
    if !cargo_success.success() {
        bail!(
            "'cargo check' exited with a non-zero exit code: {}",
            cargo_success
        );
    }

    debug!("finished 'cargo check'");
    Ok(())
}

pub fn compile() -> Result<(), failure::Error> {
    debug!("building");
    let source_project = find_folder::Search::Parents(2).for_folder("source")?;

    debug!("running 'cargo web build --target=wasm32-unknown-unknown --release'");
    let cargo_success = process::Command::new("cargo")
        .args(&[
            "web",
            "build",
            "--target=wasm32-unknown-unknown",
            "--release",
        ])
        .current_dir(&source_project)
        .spawn()?
        .wait()?;
    if !cargo_success.success() {
        bail!(
            "'cargo web' exited with a non-zero exit code: {}",
            cargo_success
        );
    }

    debug!("finished 'cargo web'");

    let target_dir = source_project.join("target/wasm32-unknown-unknown/release/");
    let wasm_file = target_dir.join("pound3pound.wasm");
    let generated_js = target_dir.join("pound3pound.js");

    let out_dir = source_project.join("../target");

    debug!("copying wasm file");

    fs::create_dir_all(&out_dir)?;

    fs::copy(wasm_file, out_dir.join("compiled.wasm"))?;

    debug!("processing js file");

    fs::write(
        out_dir.join("main.js"),
        process_js(&generated_js, &fs::read_string(&generated_js)?)?,
    )?;

    Ok(())
}

fn process_js(file_name: &Path, input: &str) -> Result<String, failure::Error> {
    // first, strip out bootstrap code which relates to the browser. We don't want
    // to run this, we just want to call `__initialize` ourself.
    //
    // TODO: this is currently quite brittle and tied to the
    // version of "cargo web"...
    let expected_prefix = r#""use strict";

if( typeof Rust === "undefined" ) {
    var Rust = {};
}

(function( root, factory ) {
    if( typeof define === "function" && define.amd ) {
        define( [], factory );
    } else if( typeof module === "object" && module.exports ) {
        module.exports = factory();
    } else {
        Rust.pound3pound = factory();
    }
}( this, function() {
    "#;

    let expected_suffix = r#"


    if( typeof window === "undefined" ) {
        const fs = require( "fs" );
        const path = require( "path" );
        const wasm_path = path.join( __dirname, "pound3pound.wasm" );
        const buffer = fs.readFileSync( wasm_path );
        const mod = new WebAssembly.Module( buffer );

        return __initialize( mod, false );
    } else {
        return fetch( "pound3pound.wasm" )
            .then( response => response.arrayBuffer() )
            .then( bytes => WebAssembly.compile( bytes ) )
            .then( mod => __initialize( mod, true ) );
    }
}));
"#;

    ensure!(
        input.starts_with(expected_prefix),
        "'cargo web' generated unexpected JS prefix! This means it's updated without \
         'cargo screeps' also having updates. Please report this issue to \
         https://github.com/daboross/screeps-in-rust-via-wasm/issues and include \
         the first ~30 lines of {}",
        file_name.display(),
    );
    ensure!(
        input.starts_with(expected_prefix),
        "'cargo web' generated unexpected JS prefix! This means it's updated without \
         'cargo screeps' also having updates. Please report this issue to \
         https://github.com/daboross/screeps-in-rust-via-wasm/issues and include \
         the last ~30 lines of {}",
        file_name.display(),
    );

    ensure!(
        input.contains("__initialize"),
        "'cargo web' generated unexpected JS output! It does not \
         include a '__initialize' function. Please report this issue to \
         https://github.com/daboross/screeps-in-rust-via-wasm/issues."
    );

    let initialize_function = &input[expected_prefix.len()..input.len() - expected_suffix.len()];

    Ok(format!(
        "{}\n{}",
        initialize_function, SCREEPS_JS_INITIALIZE_CALL
    ))
}
