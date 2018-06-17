use std::{env, path::PathBuf};

use failure;

pub fn find_project_root() -> Result<PathBuf, failure::Error> {
    let orig = env::current_dir()?;
    let mut current = orig.clone();

    loop {
        if current.join("Cargo.toml").exists() {
            return Ok(current);
        }
        let is_last = !current.pop();
        if is_last {
            bail!("could not find crate root {} or parents", orig.display());
        }
    }
}
