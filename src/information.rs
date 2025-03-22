use std::process::Command;

use sysinfo::System;

pub fn get_system_information() -> String {
    let required_information: Vec<&str> = vec![
        "system", "kernel_version", "os_version", "host_name"
    ];
    
    // Feed the system information as a background knowledge
    let mut system_information = String::new();
    for information in required_information {
        match information {
            "system" => system_information.push_str(&format!("System: {}\n", System::name().unwrap())),
            "kernel_version" => system_information.push_str(&format!("Kernel Version: {}\n", System::kernel_version().unwrap())),
            "os_version" => system_information.push_str(&format!("OS Version: {}\n", System::os_version().unwrap())),
            "host_name" => system_information.push_str(&format!("Host Name: {}\n", System::host_name().unwrap())),
            _ => {}
        }
    }
    
    system_information
}

pub fn get_available_commands() -> String {
    let available_commands: String = if cfg!(target_os = "windows") {
        // Windows system: use 'where' command to list available commands
        let output = Command::new("cmd")
            .args(&["/C", "where", "/Q", "*"])
            .output()
            .expect("Failed to execute command");
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect()
    } else {
        // Unix system: use 'compgen -c' to list available commands
        let output = Command::new("sh")
            .arg("-c")
            .arg("compgen -c")
            .output()
            .expect("Failed to execute command");
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect()
    };

    available_commands
}