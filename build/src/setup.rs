use std::{fs, io};

use {clap, failure, fern, find_folder, log, toml};

pub enum CliState {
    Build,
    BuildUpload,
}

pub fn setup_cli() -> CliState {
    let args = clap::App::new("#3# builder")
        .version("0.1")
        .author("David Ross")
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
            clap::Arg::with_name("upload")
                .short("u")
                .long("upload")
                .takes_value(false)
                .help("upload files to screeps (implies build)"),
        )
        .group(
            clap::ArgGroup::with_name("command")
                .args(&["build", "upload"])
                .multiple(true)
                .required(true),
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
        .chain(io::stdout())
        .apply()
        .unwrap();

    assert!(args.is_present("build") || args.is_present("upload"));

    if args.is_present("upload") {
        CliState::BuildUpload
    } else {
        CliState::Build
    }
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
}
impl Configuration {
    pub fn setup() -> Result<Self, failure::Error> {
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
}
