// we're literally depending on nightly only for fs_read_write, and it's quite literally a convenience feature.
// still, we're probably going to be wanting nightly anyways for best WASM support, and this does help test
// out the feature.
#![feature(fs_read_write)]
extern crate base64;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate fern;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate toml;

mod setup;
mod build;
mod upload;
mod orientation;

fn run() -> Result<(), failure::Error> {
    let state = setup::setup_cli()?;

    let root = orientation::find_project_root()?;

    match state {
        setup::CliState::Build => {
            info!("compiling...");
            build::build(&root)?;
            info!("compiled.");
        }
        setup::CliState::BuildUpload => {
            let config = setup::Configuration::setup(&root)?;

            info!("compiling...");
            build::build(&root)?;
            info!("compiled. uploading...");
            upload::upload(&root, config)?;
            info!("uploaded.");
        }
        setup::CliState::Check => {
            info!("checking...");
            build::check(&root)?;
            info!("checked.");
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e.backtrace());
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
