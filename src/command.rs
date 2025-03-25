// src/command.rs
use tokio::process::Command;
use std::io;
use std::process::Stdio;
use crate::validator;

/// Executes the given command string on Linux and returns its standard output.
///
/// Before execution, the command is validated against exclusion patterns.
/// If the command matches any dangerous pattern, it will be rejected.
pub async fn execute_command(command: &str) -> io::Result<String> {
    // First, validate the command against exclusion patterns
    if let Err(reason) = validator::validate_command(command) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("Command rejected: {}", reason)
        ));
    }

    // Split the command string into program and arguments.
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty command"));
    }
    let program = parts[0];
    let args = &parts[1..];

    // Spawn the process asynchronously.
    let output = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if output.status.success() {
        // Convert stdout from bytes to String.
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        // On error, return stderr as the error message.
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(io::Error::new(io::ErrorKind::Other, stderr))
    }
}
