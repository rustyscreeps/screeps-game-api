use std::{
    env,
    path::{Path, PathBuf},
};

use failure::format_err;

use crate::setup::CliConfig;

pub fn find_project_root(cli_config: &CliConfig) -> Result<PathBuf, failure::Error> {
    if let Some(config_path) = cli_config.config_path.as_ref() {
        // first try without canonicalization
        if let Some(noncanon_parent) = config_path.parent() {
            let noncanon_parent_cargo_toml = noncanon_parent.join("Cargo.toml");
            if noncanon_parent_cargo_toml.exists() {
                // but we do want to have our final return dir canonicalized, even if we don't
                // follow the symlink that is the config itself.
                return Ok(noncanon_parent_cargo_toml
                    .canonicalize()?
                    .parent()
                    .expect("expected path with child just pushed onto it to have parent")
                    .to_owned());
            }
        }
        return Ok(config_path
            .canonicalize()?
            .parent()
            .ok_or_else(|| format_err!("config option specified which has no parent"))?
            .to_owned());
    }

    // TODO: is it right to canonicalize here?
    let here = env::current_dir()?;
    search_dir(&here).map(Ok).unwrap_or_else(|| {
        let canon_here = here.canonicalize()?;
        search_dir(&canon_here).ok_or_else(|| {
            format_err!(
                "could not find 'screeps.toml' in {} or parents.\n\
                 Please create 'screeps.toml' in project root. (example at \
                 https://github.com/daboross/screeps-in-rust-via-wasm/\
                 blob/master/screeps-defaults.toml)",
                canon_here.display()
            )
        })
    })
}

fn search_dir(dir: &Path) -> Option<PathBuf> {
    let mut current = dir.to_owned();

    loop {
        if current.join("screeps.toml").exists() {
            return Some(current);
        }
        let has_parent = current.pop();
        if !has_parent {
            break None;
        }
    }
}
