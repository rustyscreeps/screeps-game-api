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

mod setup;
mod build;
mod upload;

fn run() -> Result<(), failure::Error> {
    let state = setup::setup_cli();

    let config = setup::Configuration::setup()?;

    info!("compiling...");

    build::compile()?;

    if let setup::CliState::BuildUpload = state {
        info!("compiled. uploading...");

        upload::upload(config)?;
    }

    info!("done.");

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e.backtrace());
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
