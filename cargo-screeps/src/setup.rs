use std::{io, path::PathBuf};

use {
    clap::{self, AppSettings},
    failure, fern, log,
};

#[derive(Clone, Debug)]
pub struct CliConfig {
    pub command: Command,
    pub config_path: Option<PathBuf>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Check,
    Build,
    Deploy,
    Upload,
    Copy,
}

fn app() -> clap::App<'static, 'static> {
    clap::App::new("cargo screeps")
        .bin_name("cargo")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            clap::SubCommand::with_name("screeps")
                .author("David Ross")
                .version(crate_version!())
                .about("Builds WASM-targetting Rust code and deploys to Screeps game servers")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    clap::Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .multiple(true),
                )
                .arg(
                    clap::Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .multiple(false)
                        .takes_value(true)
                        .value_name("CONFIG_FILE"),
                )
                .subcommand(
                    clap::SubCommand::with_name("build")
                        .about("build files, put in target/ in project root"),
                )
                .subcommand(
                    clap::SubCommand::with_name("check")
                        .about("runs 'cargo check' with appropriate target"),
                )
                .subcommand(
                    clap::SubCommand::with_name("deploy")
                        .about("run default deploy action (copy or upload)"),
                )
                .subcommand(
                    clap::SubCommand::with_name("copy")
                        .about("deploy by copying files to a local directory (implies build)"),
                )
                .subcommand(
                    clap::SubCommand::with_name("upload")
                        .about("deploy by uploading files to a remote server (implies build)"),
                ),
        )
}

pub fn setup_cli() -> Result<CliConfig, failure::Error> {
    let cargo_args = app().get_matches();

    let args = cargo_args.subcommand_matches("screeps").ok_or_else(|| {
        format_err!("expected first subcommand to be 'screeps'. please run as 'cargo screeps'")
    })?;

    let verbosity = match args.occurrences_of("verbose") {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    fern::Dispatch::new()
        .level(verbosity)
        .format(|out, message, record| out.finish(format_args!("{}: {}", record.target(), message)))
        .chain(io::stdout())
        .apply()
        .unwrap();

    let command = match args.subcommand_name() {
        Some("build") => Command::Build,
        Some("check") => Command::Check,
        Some("deploy") => Command::Deploy,
        Some("copy") => Command::Copy,
        Some("upload") => Command::Upload,
        other => panic!("unexpected subcommand {:?}", other),
    };
    let config = CliConfig {
        command,
        config_path: args.value_of("config").map(Into::into),
    };

    Ok(config)
}
