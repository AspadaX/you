use std::process::{Command, Output};

use chrono::Local;
use sysinfo::System;

pub fn get_system_information() -> String {
    let required_information: Vec<&str> =
        vec!["system", "kernel_version", "os_version", "host_name"];

    // Feed the system information as a background knowledge
    let mut system_information = String::new();
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

pub fn get_available_commands() -> String {
    let output: Output = if cfg!(target_os = "windows") {
        // Windows system: use 'where' command to list available commands
        Command::new("cmd")
            .args(&["/C", "where", "/Q", "*"])
            .output()
            .expect("Failed to execute command")
    } else {
        // Unix system: use 'compgen -c' to list available commands
        Command::new("sh")
            .arg("-c")
            .arg("compgen -c")
            .output()
            .expect("Failed to execute command")
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string() + "\n")
        .collect()
}

pub fn get_current_time() -> String {
    let now: chrono::DateTime<Local> = Local::now();
    format!("{}\n", now.format("%Y-%m-%d %H:%M:%S"))
}

pub fn get_current_directory_structure() -> String {
    let current_dir: std::path::PathBuf = std::env::current_dir().unwrap();
    let mut dir_structure = String::new();
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
