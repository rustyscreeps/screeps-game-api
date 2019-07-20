#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod build;
mod config;
mod copy;
mod orientation;
mod run;
mod setup;
mod upload;

fn main() {
    if let Err(e) = run::run() {
        eprintln!("error: {}", e);
        for cause in e.iter_causes() {
            eprintln!("  ⬑ {}", cause);
        }
        let backtrace = format!("{}", e.backtrace());
        // don't print an empty backspace line if it's not enabled.
        if backtrace.trim() != "" {
            eprintln!("{}", backtrace);
        }
        std::process::exit(1);
    }
}
