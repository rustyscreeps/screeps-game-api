use std::path::Path;

use config::{Configuration, DeployMode};

use upload::upload;

use copy::copy;

use failure;

pub fn deploy<P: AsRef<Path>>(root: P, config: Configuration) -> Result<(), failure::Error> {
    match config.mode {
        DeployMode::Upload => upload(root, config),
        DeployMode::Copy => copy(root, config),
    }
}
