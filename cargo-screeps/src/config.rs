use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use failure::{self, ResultExt};
use serde_ignored;
use toml;

#[derive(Clone, Debug, Deserialize)]
pub struct BuildConfiguration {
    #[serde(default = "BuildConfiguration::default_output_wasm_file")]
    pub output_wasm_file: PathBuf,
    #[serde(default = "BuildConfiguration::default_output_js_file")]
    pub output_js_file: PathBuf,
    #[serde(default)]
    pub initialization_header_file: Option<PathBuf>,
}

impl Default for BuildConfiguration {
    fn default() -> Self {
        BuildConfiguration {
            output_wasm_file: Self::default_output_wasm_file(),
            output_js_file: Self::default_output_js_file(),
            initialization_header_file: None,
        }
    }
}

impl BuildConfiguration {
    fn default_output_js_file() -> PathBuf {
        "main.js".into()
    }
    fn default_output_wasm_file() -> PathBuf {
        "compiled.wasm".into()
    }
}

#[derive(Clone, Debug, Deserialize)]
struct FileUploadConfiguration {
    username: String,
    password: String,
    branch: String,
    #[serde(default = "default_hostname")]
    hostname: String,
    #[serde(default)]
    ssl: Option<bool>,
    port: Option<i32>,
    #[serde(default = "default_ptr")]
    ptr: bool,
}

fn default_hostname() -> String {
    "screeps.com".to_owned()
}

fn default_ptr() -> bool {
    false
}

#[derive(Clone, Debug)]
pub struct UploadConfiguration {
    pub username: String,
    pub password: String,
    pub hostname: String,
    pub branch: String,
    pub ssl: bool,
    pub port: i32,
    pub ptr: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CopyConfiguration {
    pub destination: PathBuf,
    pub branch: String,
    #[serde(default = "default_prune")]
    pub prune: bool,
}

fn default_prune() -> bool {
    false
}

#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DeployMode {
    Copy,
    Upload,
}

#[derive(Clone, Debug, Deserialize)]
struct FileConfiguration {
    default_deploy_mode: Option<DeployMode>,
    #[serde(default)]
    build: BuildConfiguration,
    upload: Option<FileUploadConfiguration>,
    copy: Option<CopyConfiguration>,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub default_deploy_mode: Option<DeployMode>,
    pub build: BuildConfiguration,
    pub copy: Option<CopyConfiguration>,
    pub upload: Option<UploadConfiguration>,
}

impl UploadConfiguration {
    fn new(config: FileUploadConfiguration) -> Result<UploadConfiguration, failure::Error> {
        let FileUploadConfiguration {
            username,
            password,
            branch,
            hostname,
            ssl,
            port,
            ptr,
        } = config;

        let ssl = ssl.unwrap_or_else(|| hostname == "screeps.com");
        let port = port.unwrap_or_else(|| if ssl { 443 } else { 80 });

        Ok(UploadConfiguration {
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

impl Configuration {
    fn new(config: FileConfiguration) -> Result<Configuration, failure::Error> {
        Ok(Configuration {
            default_deploy_mode: config.default_deploy_mode,
            build: config.build,
            upload: match config.upload {
                Some(upload_config) => Some(UploadConfiguration::new(upload_config)?),
                None => None,
            },
            copy: config.copy,
        })
    }
}

impl Configuration {
    pub fn read<P: AsRef<Path>>(config_file: P) -> Result<Self, failure::Error> {
        let config_file = config_file.as_ref();
        ensure!(
            config_file.exists(),
            "expected configuration to exist at {}",
            config_file.display(),
        );

        let config_str = {
            use std::io::Read;
            let mut buf = String::new();
            fs::File::open(config_file)
                .context("opening config file")?
                .read_to_string(&mut buf)
                .context("reading config file")?;
            buf
        };

        let mut unused_paths = BTreeSet::new();

        let file_config: FileConfiguration =
            serde_ignored::deserialize(&mut toml::Deserializer::new(&config_str), |unused_path| {
                unused_paths.insert(unused_path.to_string());
            })
            .context("deserializing config")?;

        for path in &unused_paths {
            warn!("unused configuration path: {}", path)
        }

        Configuration::new(file_config)
    }
}
