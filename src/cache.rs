use std::{fs::{create_dir, read_dir, DirEntry}, path::PathBuf};

use anyhow::Result;

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

        Ok(
            Self { scripts }
        )
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
    
    pub fn add_new_script(&mut self, script_name: &str, script_content: &str) -> Result<()> {
        Ok(())
    }
}
