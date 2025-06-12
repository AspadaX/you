use anyhow::Result;
use chrono::Local;
use sysinfo::System;

use crate::configurations::Configurations;

pub struct ContextualInformation {
    system_information: String,
    current_time: String,
    current_directory_structure: String,
    configurations: Configurations,
}

impl ContextualInformation {
    /// Fetches system information and loads configurations using the load() method.
    pub fn new() -> Result<Self> {
        Ok(
            Self {
                system_information: get_system_information(),
                current_time: get_current_time(),
                current_directory_structure: get_current_directory_structure(),
                configurations: Configurations::load()?,
            }
        )
    }
    
    /// Get the required contextual information for agents:
    /// - System specs
    /// - Current time
    /// - Current directory structure
    /// - User configurations
    pub fn get_contextual_information(&self) -> Result<String> {
        let mut contextual_information: String = String::new();
        
        // Inject the system information
        contextual_information.push_str("Environment:\n");
        contextual_information.push_str(&self.system_information);
        contextual_information.push_str("Current Working Directory: ");
        contextual_information.push_str(std::env::current_dir()?.to_str().unwrap());
        contextual_information.push_str("\n");
        contextual_information.push_str("Current Working Directory Sturcture: ");
        contextual_information.push_str(&self.current_directory_structure);
        contextual_information.push_str("\n");
        contextual_information.push_str("Current Date and Time: ");
        contextual_information.push_str(&self.current_time);
        contextual_information.push_str("\n");
        contextual_information.push_str("User preferred CLIs: ");
        contextual_information.push_str(&format!("{}", &self.configurations.get_preferred_clis()));
        contextual_information.push_str("\n");
        
        Ok(contextual_information)
    }
}

pub fn get_system_information() -> String {
    let required_information: Vec<&str> =
        vec!["system", "kernel_version", "os_version", "host_name"];

    // Feed the system information as a background knowledge
    let mut system_information: String = String::new();
    for information in required_information {
        match information {
            "system" => {
                system_information.push_str(&format!("System: {}\n", System::name().unwrap()))
            }
            "kernel_version" => system_information.push_str(&format!(
                "Kernel Version: {}\n",
                System::kernel_version().unwrap()
            )),
            "os_version" => system_information
                .push_str(&format!("OS Version: {}\n", System::os_version().unwrap())),
            "host_name" => system_information
                .push_str(&format!("Host Name: {}\n", System::host_name().unwrap())),
            _ => {}
        }
    }

    system_information
}

pub fn get_current_time() -> String {
    let now: chrono::DateTime<Local> = Local::now();
    format!("{}\n", now.format("%Y-%m-%d %H:%M:%S"))
}

pub fn get_current_directory_structure() -> String {
    let current_dir: std::path::PathBuf = std::env::current_dir().unwrap();
    let mut dir_structure: String = String::new();
    for entry in std::fs::read_dir(current_dir).unwrap() {
        let entry: std::fs::DirEntry = entry.unwrap();
        let metadata: std::fs::Metadata = entry.metadata().unwrap();
        let file_type: &str = if metadata.is_dir() { "dir" } else { "file" };
        dir_structure.push_str(&format!(
            "{} {}\n",
            file_type,
            entry.file_name().to_string_lossy()
        ));
    }

    dir_structure
}
