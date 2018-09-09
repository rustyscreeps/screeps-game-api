use std::{env, path::PathBuf};

use failure;

use setup::CliConfig;

pub fn find_project_root(cli_config: &CliConfig) -> Result<PathBuf, failure::Error> {
    if let Some(config_path) = cli_config.config_path.as_ref() {
        return Ok(config_path
            .canonicalize()?
            .parent()
            .ok_or_else(|| format_err!("config option specified which has no parent"))?
            .to_owned());
    }

    // TODO: is it right to canonicalize here?
    let original = env::current_dir()?.canonicalize()?;
    let mut current = original.clone();

    loop {
        if current.join("screeps.toml").exists() {
            return Ok(current);
        }
        let is_last = current.pop();
        if is_last {
            bail!(
                "could not find 'screeps.toml' in {} or parents.\n\
                 Please create 'screeps.toml' in project root. (template at \
                 https://github.com/daboross/screeps-in-rust-via-wasm/blob/master/screeps.toml)",
                original.display()
            );
        }
    }
}
