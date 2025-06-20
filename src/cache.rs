use std::{
    fs::{DirEntry, File, create_dir, read_dir},
    io::Write,
    path::PathBuf,
};

use anyhow::{Result, anyhow};

use crate::{
    constants::YOU_CACHE_DIRECTORY,
    traits::{GlobalResourceInitialization, acquire_you_home_directory},
};

#[derive(Debug, Clone)]
pub struct Cache {
    scripts: Vec<PathBuf>,
}

impl GlobalResourceInitialization for Cache {
    fn initialize() -> Result<()> {
        let you_cache_directory: PathBuf = acquire_you_home_directory()?.join(YOU_CACHE_DIRECTORY);

        if !you_cache_directory.exists() {
            create_dir(&you_cache_directory)?;
        }

        Ok(())
    }

    fn load() -> Result<Self>
    where
        Self: Sized,
    {
        let mut scripts: Vec<PathBuf> = Vec::new();
        for file in read_dir(acquire_you_home_directory()?.join(YOU_CACHE_DIRECTORY))? {
            let file: DirEntry = file?;
            if file.metadata().unwrap().is_file() {
                let filename: String = file.file_name().to_string_lossy().to_string();
                if filename.ends_with(".sh") {
                    scripts.push(file.path());
                }
            }
        }

        Ok(Self { scripts })
    }
}

impl Cache {
    pub fn search(&self, query: &str) -> Option<&PathBuf> {
        for script in self.scripts.iter() {
            let script_name: &str = script.file_stem().unwrap().to_str().unwrap();
            if query == script_name {
                return Some(&script);
            }
        }

        None
    }

    pub fn add_new_script(&self, script_name: &str, script_content: &str) -> Result<()> {
        let you_cache_directory: PathBuf = acquire_you_home_directory()?.join(YOU_CACHE_DIRECTORY);

        let mut file: File =
            std::fs::File::create_new(you_cache_directory.join(format!("{}.sh", script_name)))?;
        file.write(script_content.as_bytes())?;

        Ok(())
    }

    pub fn delete_script(&self, script_name: &str) -> Result<()> {
        for script in self.scripts.iter() {
            let current_script_name: &str = script.file_stem().unwrap().to_str().unwrap();
            if current_script_name == script_name {
                std::fs::remove_file(script)?;
                break;
            }
        }

        Err(anyhow!("Script '{}' not found", script_name))
    }
}
