use std::io;

use {clap, failure, fern, log, orientation, pathdiff};

use config::{Configuration, DeployMode};

use std::env;

pub enum CliState {
    Check,
    Build,
    Deploy,
}

pub fn setup_cli() -> Result<(CliState, Configuration), failure::Error> {
    let cwd = env::current_dir()?;
    let default_config: Option<String> = orientation::find_project_root().ok().and_then(|r| {
        pathdiff::diff_paths(&r, &cwd).map(|d| format!("{}", d.join("screeps.toml").display()))
    });
    let cargo_args =
        clap::App::new("cargo screeps")
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
                    ).arg({
                        let cfg_arg = clap::Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .multiple(false)
                            .takes_value(true)
                            .value_name("FILE");
                        if let Some(ref cfg) = default_config {
                            cfg_arg.default_value(cfg)
                        } else {
                            cfg_arg
                        }
                    }).subcommand(
                        clap::SubCommand::with_name("build")
                            .about("build files, put in target/ in project root"),
                    ).subcommand(
                        clap::SubCommand::with_name("check")
                            .about("runs 'cargo check' with appropriate target"),
                    ).subcommand(
                        clap::SubCommand::with_name("deploy")
                            .about("deploy files to a screeps server (implies build)"),
                    ).subcommand(clap::SubCommand::with_name("copy").about(
                        "copy files to a screeps client deployment directory (implies build)",
                    )).subcommand(
                        clap::SubCommand::with_name("upload")
                            .about("upload files to a screeps server (implies build)"),
                    ),
            ).get_matches();

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

    let mut config = Configuration::read(args.value_of("config").unwrap())?;
    let state = if args.subcommand_matches("check").is_some() {
        CliState::Check
    } else if args.subcommand_matches("deploy").is_some() {
        CliState::Deploy
    } else if args.subcommand_matches("copy").is_some() {
        config.mode = DeployMode::Copy;
        CliState::Deploy
    } else if args.subcommand_matches("upload").is_some() {
        config.mode = DeployMode::Upload;
        CliState::Deploy
    } else {
        CliState::Build
    };

    Ok((state, config))
}
