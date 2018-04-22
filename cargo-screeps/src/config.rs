use failure::{self, ResultExt};

use toml;

use std::{convert::TryFrom,
          fs,
          path::{Path, PathBuf}};

#[derive(Clone, Debug, Deserialize)]
struct FileConfiguration {
    #[serde(default)]
    mode: DeployMode,
    #[serde(default)]
    branch: Option<String>,
    #[serde(default)]
    output_wasm_file: Option<PathBuf>,
    #[serde(default)]
    output_js_file: Option<PathBuf>,
    #[serde(flatten)]
    old_upload: FileUploadConfiguration,
    #[serde(default)]
    upload: Option<FileUploadConfiguration>,
    #[serde(default)]
    copy: Option<FileCopyConfiguration>,
}

#[derive(Clone, Debug, Deserialize)]
struct FileUploadConfiguration {
    // These two shouldn't be optional, but will be for backwards compat.
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    password: Option<String>,

    #[serde(default)]
    hostname: Option<String>,
    #[serde(default)]
    ssl: Option<bool>,
    #[serde(default)]
    port: Option<i32>,
    #[serde(default)]
    ptr: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct FileCopyConfiguration {
    dest: PathBuf,
    #[serde(default)]
    prune: bool,
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DeployMode {
    Copy,
    Upload,
}

impl Default for DeployMode {
    fn default() -> DeployMode {
        DeployMode::Upload
    }
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub branch: String,
    pub output_wasm_file: PathBuf,
    pub output_js_file: PathBuf,
    pub deploy: DeployConfiguration,
}

#[derive(Clone, Debug)]
pub enum DeployConfiguration {
    Copy(CopyConfiguration),
    Upload(UploadConfiguration),
}

#[derive(Clone, Debug)]
pub struct UploadConfiguration {
    pub username: String,
    pub password: String,
    pub hostname: String,
    pub ssl: bool,
    pub port: i32,
    pub ptr: bool,
}

#[derive(Clone, Debug)]
pub struct CopyConfiguration {
    pub dest: PathBuf,
    pub prune: bool,
}

#[derive(Fail, Debug)]
pub enum ConfigError {
    #[fail(display = "missing {} section", _0)]
    MissingSection(&'static str),
    #[fail(display = "missing username")]
    MissingUsername,
    #[fail(display = "missing password")]
    MissingPassword,
}

impl TryFrom<FileConfiguration> for Configuration {
    type Error = ConfigError;

    fn try_from(value: FileConfiguration) -> Result<Configuration, Self::Error> {
        let FileConfiguration {
            mode,
            branch,
            old_upload,
            upload,
            copy,
            output_wasm_file,
            output_js_file,
        } = value;

        let deploy = match mode {
            DeployMode::Copy => {
                let FileCopyConfiguration { dest, prune } =
                    copy.ok_or(ConfigError::MissingSection("copy"))?;

                DeployConfiguration::Copy(CopyConfiguration { dest, prune })
            }
            DeployMode::Upload => {
                let FileUploadConfiguration {
                    username,
                    password,
                    hostname,
                    ssl,
                    port,
                    ptr,
                } = upload.unwrap_or(old_upload);

                let hostname = hostname.unwrap_or_else(|| "screeps.com".into());
                let ssl = ssl.unwrap_or_else(|| hostname == "screeps.com");
                let port = port.unwrap_or_else(|| if ssl { 443 } else { 80 });
                let username = username.ok_or(ConfigError::MissingUsername)?;
                let password = password.ok_or(ConfigError::MissingPassword)?;

                DeployConfiguration::Upload(UploadConfiguration {
                    username,
                    password,
                    hostname,
                    ssl,
                    port,
                    ptr,
                })
            }
        };

        let branch = branch.unwrap_or_else(|| "default".into());
        let output_js_file = output_js_file.unwrap_or_else(|| "main.js".into());
        let output_wasm_file = output_wasm_file.unwrap_or_else(|| "compiled.wasm".into());

        Ok(Configuration {
            branch,
            deploy,
            output_js_file,
            output_wasm_file,
        })
    }
}
impl Configuration {
    pub fn read(root: &Path) -> Result<Self, failure::Error> {
        let config_file = root.join("screeps.toml");
        ensure!(
            config_file.exists(),
            "expected screeps.toml to exist in {}",
            root.display(),
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

        let file_config: FileConfiguration =
            toml::from_str(&config_str).context("deserializing config")?;

        Ok(Configuration::try_from(file_config)?)
    }
}
