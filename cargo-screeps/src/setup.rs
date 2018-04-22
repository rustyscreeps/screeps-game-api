use std::io;

use {clap, failure, fern, log};

pub enum CliState {
    Check,
    Build,
    BuildUpload,
}

pub fn setup_cli() -> Result<CliState, failure::Error> {
    let cargo_args = clap::App::new("cargo screeps")
        .bin_name("cargo")
        .subcommand(
            clap::SubCommand::with_name("screeps")
                .author("David Ross")
                .version(crate_version!())
                .about("Builds WASM-targetting Rust code and deploys to Screeps game servers")
                .arg(
                    clap::Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .multiple(true),
                )
                .arg(
                    clap::Arg::with_name("build")
                        .short("b")
                        .long("build")
                        .takes_value(false)
                        .help("build files, put in target/ in project root"),
                )
                .arg(
                    clap::Arg::with_name("check")
                        .short("c")
                        .long("check")
                        .takes_value(false)
                        .help("runs 'cargo check' with appropriate target"),
                )
                .arg(
                    clap::Arg::with_name("upload")
                        .short("u")
                        .long("upload")
                        .takes_value(false)
                        .help("upload files to screeps (implies build)"),
                )
                .group(
                    clap::ArgGroup::with_name("command")
                        .args(&["build", "upload", "check"])
                        .multiple(false)
                        .required(true),
                ),
        )
        .get_matches();

    let args = cargo_args.subcommand_matches("screeps").ok_or_else(|| {
        format_err!("expected first subcommand to be 'screeps' (please run as 'cargo screeps')")
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

    assert!(args.is_present("check") || args.is_present("build") || args.is_present("upload"));

    let state = if args.is_present("check") {
        CliState::Check
    } else if args.is_present("upload") {
        CliState::BuildUpload
    } else {
        CliState::Build
    };

    Ok(state)
}
