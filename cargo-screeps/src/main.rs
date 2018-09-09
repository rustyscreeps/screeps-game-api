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
mod copy;
mod orientation;
mod run;
mod setup;
mod upload;

fn main() {
    if let Err(e) = run::run() {
        // eprintln!("{}", e.backtrace());
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
