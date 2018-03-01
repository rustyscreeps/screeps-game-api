use std::{env, path::PathBuf};

use failure;

pub fn find_project_root() -> Result<PathBuf, failure::Error> {
    // TODO: is it right to canonicalize here?
    let mut current = env::current_dir()?.canonicalize()?;

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
                current.display()
            );
        }
    }
}
