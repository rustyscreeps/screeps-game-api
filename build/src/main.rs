#![feature(fs_read_write)] // it's just a convenience thing, but hey, we're helping try it out!
extern crate base64;
extern crate clap;
#[macro_use]
extern crate failure;
extern crate fern;
extern crate find_folder;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate toml;

use std::{fs, process};
use std::collections::HashMap;

fn cli_setup() {
    let args = clap::App::new("#3# builder")
        .version("0.1")
        .author("David Ross")
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true),
        )
        .get_matches();
    let verbosity = match args.occurrences_of("verbose") {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    fern::Dispatch::new()
        .level(verbosity)
        .format(|out, message, record| out.finish(format_args!("{}: {}", record.target(), message)))
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}

fn default_hostname() -> String {
    "screeps.com".to_owned()
}

fn default_ptr() -> bool {
    false
}
fn default_branch() -> String {
    "default".to_owned()
}

#[derive(Deserialize)]
struct FileConfiguration {
    username: String,
    password: String,
    #[serde(default = "default_branch")]
    branch: String,
    #[serde(default = "default_hostname")]
    hostname: String,
    #[serde(default)]
    ssl: Option<bool>,
    port: Option<i32>,
    #[serde(default = "default_ptr")]
    ptr: bool,
}

struct Configuration {
    username: String,
    password: String,
    branch: String,
    hostname: String,
    ssl: bool,
    port: i32,
    ptr: bool,
}

fn config_setup() -> Result<Configuration, failure::Error> {
    let parent_dir = find_folder::Search::Parents(2)
        .for_folder("source")?
        .join("../");
    let config_file = parent_dir.join("config.toml");
    ensure!(
        config_file.exists(),
        "please copy config-defaults.toml to config.toml in the project root"
    );
    let file_config = toml::from_str(&fs::read_string(config_file)?)?;

    let FileConfiguration {
        username,
        password,
        branch,
        hostname,
        ssl,
        port,
        ptr,
    } = file_config;

    let ssl = ssl.unwrap_or_else(|| hostname == "screeps.com");
    let port = port.unwrap_or_else(|| if ssl { 443 } else { 80 });

    Ok(Configuration {
        username,
        password,
        branch,
        hostname,
        ssl,
        port,
        ptr,
    })
}

fn build() -> Result<(), failure::Error> {
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
        process_js(&fs::read_string(generated_js)?)?,
    )?;

    Ok(())
}

fn process_js(input: &str) -> Result<String, failure::Error> {
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
        "expected 'cargo web' generated JS to start with known JS"
    );
    ensure!(
        input.ends_with(expected_suffix),
        "expected 'cargo web' generated JS to end with known JS"
    );

    let initialize_function = &input[expected_prefix.len()..input.len() - expected_suffix.len()];

    let wrapping_js_file = find_folder::Search::Parents(2)
        .for_folder("js")?
        .join("main.js");

    let wrapper_js = fs::read_string(wrapping_js_file)?;

    Ok(format!("{}\n{}", initialize_function, wrapper_js))
}

fn upload(config: Configuration) -> Result<(), failure::Error> {
    let target_dir = find_folder::Search::Parents(2)
        .for_folder("source")?
        .join("../target");
    let mut files = HashMap::new();
    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();

        ensure!(
            entry.file_type()?.is_file(),
            "non-file found in 'target' dir: {}",
            path.display()
        );

        if let (Some(name), Some(extension)) = (path.file_stem(), path.extension()) {
            let contents = if extension == "js" {
                let data = fs::read_string(&path)?;
                serde_json::Value::String(data)
            } else if extension == "wasm" {
                let data = base64::encode(&fs::read(&path)?);
                json!({ "binary": data })
            } else {
                bail!("non-js non-wasm file found in target/");
            };

            files.insert(name.to_string_lossy().into_owned(), contents);
        }
    }

    let client = reqwest::Client::new();

    let url = format!(
        "{}://{}:{}/{}",
        if config.ssl { "https" } else { "http" },
        config.hostname,
        config.port,
        if config.ptr {
            "ptr/api/user/code"
        } else {
            "api/user/code"
        }
    );

    #[derive(Serialize)]
    struct RequestData {
        modules: HashMap<String, serde_json::Value>,
        branch: String,
    }

    let mut response = client
        .post(&*url)
        .basic_auth(config.username, Some(config.password))
        .header(reqwest::header::ContentType::json())
        .body(serde_json::to_string(&RequestData {
            modules: files,
            branch: config.branch.clone(),
        })?)
        .send()?;

    let response_text = response.text()?;

    ensure!(
        response.status().is_success(),
        "uploading to '{}' failed: {}",
        response.url(),
        response_text,
    );

    debug!("upload finished: {}", response_text);
    debug!("response: {:#?}", response);

    let response_json: serde_json::Value = response.json()?;

    if let Some(s) = response_json.get("error") {
        bail!(
            "error sending to branch '{}' of '{}': {}",
            config.branch,
            response.url(),
            s
        );
    }

    Ok(())
}

fn run() -> Result<(), failure::Error> {
    cli_setup();

    let config = config_setup()?;

    info!("compiling...");

    build()?;

    info!("compiled. uploading...");

    upload(config)?;

    info!("done.");

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e.backtrace());
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
