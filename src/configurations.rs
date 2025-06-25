use std::io::Write;
use std::{fmt::Display, fs::File, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::constants::CONFIGURATIONS_JSON;
use crate::traits::{GlobalResourceInitialization, acquire_you_home_directory};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PreferredCLI {
    name: String,
    preferred_for: String,
}

impl Display for PreferredCLI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(&self).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configurations {
    #[serde(default)]
    pub enable_cache: bool,
    preferred_clis: Vec<PreferredCLI>,
}

impl Default for Configurations {
    fn default() -> Self {
        Self {
            enable_cache: false,
            preferred_clis: vec![],
        }
    }
}

impl Display for Configurations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(&self).unwrap())
    }
}

impl Configurations {
    pub fn get_preferred_clis(&self) -> String {
        let mut prompt: String = String::new();
        for preferred_cli in self.preferred_clis.iter() {
            prompt.push_str(&format!(
                "The user prefers using {} for {}",
                preferred_cli.name, preferred_cli.preferred_for
            ));
        }

        prompt
    }
}

impl GlobalResourceInitialization for Configurations {
    fn initialize() -> Result<()> {
        let configurations_directory: PathBuf =
            acquire_you_home_directory()?.join(CONFIGURATIONS_JSON);

        if !configurations_directory.exists() {
            let mut new_configuration: File = std::fs::File::create_new(&configurations_directory)?;
            new_configuration
                .write(serde_json::to_string_pretty(&Configurations::default())?.as_bytes())?;
        }

        Ok(())
    }

    fn load() -> Result<Self>
    where
        Self: Sized,
    {
        // Get the configuration json's filepath before converting
        let configuration_string: String =
            std::fs::read_to_string(acquire_you_home_directory()?.join(CONFIGURATIONS_JSON))?;

        Ok(serde_json::from_str(&configuration_string)?)
    }
}
