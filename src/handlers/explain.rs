use crate::error::{AskError, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::process::Command;

static EXPLAIN_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)(explain|describe)\s+(\w+)").unwrap());

pub fn handle(query: &str) -> Result<()> {
    let caps = EXPLAIN_PATTERN
        .captures(query)
        .ok_or_else(|| AskError::Generic(format!("Couldn't parse command from: {}", query)))?;

    let cmd = caps
        .get(2)
        .map(|m| m.as_str())
        .ok_or_else(|| AskError::Generic("No command found".to_string()))?;

    // Check if command exists
    if !command_exists(cmd) {
        return Err(AskError::CommandNotFound(cmd.to_string()));
    }

    // Try --help first
    let output = Command::new(cmd).arg("--help").output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().take(20).collect();
        println!("{}", lines.join("\n"));

        let total_lines = stdout.lines().count();
        if total_lines > 20 {
            println!("\n... (run '{} --help' for full output)", cmd);
        }
    } else {
        // Fall back to man page
        let status = Command::new("man").arg(cmd).status()?;
        if !status.success() {
            println!("No help available for: {}", cmd);
        }
    }

    Ok(())
}

fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
