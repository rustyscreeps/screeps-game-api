extern crate base64;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate fern;
#[macro_use]
extern crate log;
extern crate pathdiff;
extern crate regex;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate toml;

mod build;
mod config;
mod orientation;
mod setup;
mod upload;

fn run() -> Result<(), failure::Error> {
    let (state, config) = setup::setup_cli()?;

    let root = orientation::find_project_root()?;

    match state {
        setup::CliState::Build => {
            info!("compiling...");
            build::build(&root, &config)?;
            info!("compiled.");
        }
        setup::CliState::BuildUpload => {
            info!("compiling...");
            build::build(&root, &config)?;
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
        // eprintln!("{}", e.backtrace());
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
