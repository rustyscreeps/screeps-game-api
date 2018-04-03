use std::{fs, io, path::{Path, PathBuf}};

use {clap, failure, fern, log, toml};

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

fn default_hostname() -> String {
    "screeps.com".to_owned()
}

fn default_ptr() -> bool {
    false
}

fn default_branch() -> String {
    "default".to_owned()
}

fn default_wasm_file() -> PathBuf {
    "compiled.wasm".into()
}

fn default_js_file() -> PathBuf {
    "main.js".into()
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
    #[serde(default = "default_wasm_file")]
    output_wasm_file: PathBuf,
    #[serde(default = "default_js_file")]
    output_js_file: PathBuf,
}

// separate structure so we can have defaults based off of other config values

#[derive(Debug, Clone)]
pub struct Configuration {
    pub username: String,
    pub password: String,
    pub branch: String,
    pub hostname: String,
    pub ssl: bool,
    pub port: i32,
    pub ptr: bool,
    pub output_wasm_file: PathBuf,
    pub output_js_file: PathBuf,
}

impl Configuration {
    pub fn setup(root: &Path) -> Result<Self, failure::Error> {
        let config_file = root.join("screeps.toml");
        ensure!(
            config_file.exists(),
            "expected screeps.toml to exist in {}",
            root.display()
        );

        let config_str = {
            use std::io::Read;
            let mut buf = String::new();
            fs::File::open(config_file)?.read_to_string(&mut buf)?;
            buf
        };

        let file_config = toml::from_str(&config_str)?;

        let FileConfiguration {
            username,
            password,
            branch,
            hostname,
            ssl,
            port,
            ptr,
            output_js_file,
            output_wasm_file,
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
            output_js_file,
            output_wasm_file,
        })
    }
}
