use std::path::Path;

use failure::format_err;
use log::*;

use crate::{
    build,
    config::{self, Configuration},
    copy, orientation, setup, upload,
};

pub fn run() -> Result<(), failure::Error> {
    let cli_config = setup::setup_cli()?;

    let root = orientation::find_project_root(&cli_config)?;
    let config_path = cli_config
        .config_path
        .unwrap_or_else(|| root.join("screeps.toml").to_owned());

    let config = config::Configuration::read(&config_path)?;

    debug!(
        "Running {:?} at {:?} using config {:?} with values {:#?}",
        cli_config.command, root, config_path, config
    );

    match cli_config.command {
        setup::Command::Build => run_build(&root, &config)?,
        setup::Command::Check => run_check(&root)?,
        setup::Command::Upload => {
            run_build(&root, &config)?;
            run_upload(&root, &config)?;
        }
        setup::Command::Copy => {
            run_build(&root, &config)?;
            run_copy(&root, &config)?;
        }
        setup::Command::Deploy => {
            run_build(&root, &config)?;
            match config.default_deploy_mode.ok_or_else(|| {
                format_err!("must have default_deploy_mode set to use 'cargo screeps deploy'")
            })? {
                config::DeployMode::Upload => run_upload(&root, &config)?,
                config::DeployMode::Copy => run_copy(&root, &config)?,
            }
        }
    }

    Ok(())
}

fn run_build(root: &Path, config: &Configuration) -> Result<(), failure::Error> {
    info!("compiling...");
    build::build(root, config)?;
    info!("compiled.");

    Ok(())
}

fn run_check(root: &Path) -> Result<(), failure::Error> {
    info!("checking...");
    build::check(root)?;
    info!("checked.");

    Ok(())
}

fn run_copy(root: &Path, config: &Configuration) -> Result<(), failure::Error> {
    info!("copying...");
    copy::copy(root, config)?;
    info!("copied.");

    Ok(())
}

fn run_upload(root: &Path, config: &Configuration) -> Result<(), failure::Error> {
    info!("uploading...");
    upload::upload(root, config)?;
    info!("uploaded.");

    Ok(())
}
