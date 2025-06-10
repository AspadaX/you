use std::io::Write;
use std::{fmt::Display, fs::File, path::PathBuf};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::constants::{CONFIGURATIONS_JSON, YOU_HOME_DIRECTORY};

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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Configurations {
    preferred_clis: Vec<PreferredCLI>
}

impl Display for Configurations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(&self).unwrap())
    }
}

impl Configurations {
    // For running on the first time the cli starts
    pub fn initialize() -> Result<()> {
        let home_directory: PathBuf = match dirs::home_dir() {
            Some(result) => result.join(YOU_HOME_DIRECTORY),
            None => return Err(anyhow!("No home directory is found"))
        };
        
        if !home_directory.exists() {
            std::fs::create_dir(&home_directory)?;
            let mut new_configuration: File = std::fs::File::create_new(&home_directory.join(CONFIGURATIONS_JSON))?;
            new_configuration.write(
                serde_json::to_string_pretty(&Configurations::default())?.as_bytes()
            )?;
            
            return Ok(());
        }
        
        Ok(())
    }
    
    // Load configurations from the default position
    pub fn load() -> Result<Self> {
        let home_directory: PathBuf = match dirs::home_dir() {
            Some(result) => result.join(YOU_HOME_DIRECTORY),
            None => return Err(anyhow!("No home directory is found"))
        };
        
        // Get the configuration json's filepath before converting
        let configuration_string: String = std::fs::read_to_string(
            home_directory.join(CONFIGURATIONS_JSON)
        )?;
        
        Ok(serde_json::from_str(&configuration_string)?)
    }
    
    pub fn get_preferred_clis(&self) -> String {
        let mut prompt: String = String::new();
        for preferred_cli in self.preferred_clis.iter() {
            prompt.push_str(
                &format!("The user prefers using {} for {}", preferred_cli.name, preferred_cli.preferred_for)
            );
        }
        
        prompt
    }
}
