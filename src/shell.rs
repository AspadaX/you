/// Execute a shell script with the specified execution context
pub fn execute_shell_script_with_context(
    shell_script: &str, 
    args: &[String], 
    context: ExecutionContext
) -> Result<(), Error> {
    let script_path: &std::path::Path = std::path::Path::new(shell_script);
    
    // Determine the working directory based on the execution context
    let working_dir = match context {
        ExecutionContext::ScriptDirectory => {
            script_path.parent().unwrap_or_else(|| std::path::Path::new("."))
        },
        ExecutionContext::CurrentWorkingDirectory => {
            std::path::Path::new(".")
        },
    };

    if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", shell_script]).current_dir(working_dir);
        // Add additional arguments if provided
        if !args.is_empty() {
            cmd.args(args);
        }
        
        match cmd.status() {
            Ok(status) if !status.success() => {
                return Err(anyhow!(
                    "Windows CMD interpreter exited with a non-zero status"
                ));
            }
            Ok(_) => {},
            Err(e) => {
                return Err(anyhow!("Failed to start Windows CMD interpreter: {}", e));
            }
        }
        
        return Ok(());
    }

    let mut cmd = Command::new("sh");
    cmd.arg(shell_script).current_dir(working_dir);
    // Add additional arguments if provided
    if !args.is_empty() {
        cmd.args(args);
    }
    
    match cmd.status() {
        Ok(status) if !status.success() => {
            return Err(anyhow!("Shell interpreter exited with a non-zero status"));
        }
        Ok(_) => {},
        Err(e) => {
            return Err(anyhow!("Failed to start shell interpreter: {}", e));
        }
    }

    Ok(())
}