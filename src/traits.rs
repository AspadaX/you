use std::path::PathBuf;

use anyhow::{Result, anyhow};

use crate::constants::YOU_HOME_DIRECTORY;

pub fn acquire_you_home_directory() -> Result<PathBuf> {
    let you_home_directory = match dirs::home_dir() {
        Some(result) => result.join(YOU_HOME_DIRECTORY),
        None => return Err(anyhow!("No home directory is found")),
    };

    if !you_home_directory.exists() {
        std::fs::create_dir(&you_home_directory)?;
    }

    Ok(you_home_directory)
}

/// Impl this to have access to the home directory's resources
pub trait GlobalResourceInitialization {
    // For running on the first time the cli starts
    fn initialize() -> Result<()>;

    // Load resources from the default position
    fn load() -> Result<Self>
    where
        Self: Sized;
}
