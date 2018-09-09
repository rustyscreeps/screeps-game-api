use std::path::{Path, PathBuf};

use std::fs;

use std::collections::HashSet;

use failure;

use config::Configuration;

pub fn copy<P: AsRef<Path>>(root: P, config: &Configuration) -> Result<(), failure::Error> {
    let root = root.as_ref();
    let copy_config = config.copy.as_ref().ok_or_else(|| {
        format_err!("must include [copy] section in configuration to deploy using copy")
    })?;

    let output_dir = copy_config.destination.join(&copy_config.branch);

    fs::create_dir_all(&output_dir)?;

    let target_dir = root.join("target");

    let mut deployed: HashSet<PathBuf> = HashSet::new();

    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();

        if let (Some(name), Some(extension)) = (path.file_name(), path.extension()) {
            if extension == "js" {
                fs::copy(&path, output_dir.join(name))?;
                deployed.insert(name.into());
            } else if extension == "wasm" {
                fs::copy(&path, output_dir.join(name))?;
                deployed.insert(name.into());
            } else {
                continue;
            };
        }
    }

    if copy_config.prune {
        for entry in fs::read_dir(output_dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(name) = path.file_name().map(PathBuf::from) {
                if !deployed.contains(&name) {
                    fs::remove_file(path)?;
                }
            }
        }
    }

    Ok(())
}
