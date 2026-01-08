use crate::error::{AskError, Result};
use crate::intent::SystemResource;
use once_cell::sync::Lazy;
use regex::Regex;
use std::process::Command;

static PORT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)port\s*(\d+)").unwrap());

pub fn handle(args: &str) -> Result<()> {
    let resource = SystemResource::from_str(args.trim())
        .ok_or_else(|| AskError::UnknownResource(args.to_string()))?;

    match resource {
        SystemResource::Disk => run_command("df", &["-h"]),
        SystemResource::Memory => {
            #[cfg(target_os = "macos")]
            return run_command("vm_stat", &[]);
            #[cfg(target_os = "linux")]
            return run_command("free", &["-h"]);
            #[cfg(not(any(target_os = "macos", target_os = "linux")))]
            return Err(AskError::Generic("Unsupported platform".to_string()));
        }
        SystemResource::Cpu => {
            #[cfg(target_os = "macos")]
            return run_command("sysctl", &["-n", "machdep.cpu.brand_string"]);
            #[cfg(target_os = "linux")]
            return run_command("lscpu", &[]);
            #[cfg(not(any(target_os = "macos", target_os = "linux")))]
            return Err(AskError::Generic("Unsupported platform".to_string()));
        }
        SystemResource::Ports => {
            #[cfg(target_os = "macos")]
            return run_command("netstat", &["-an"]);
            #[cfg(target_os = "linux")]
            return run_command_or_fallback("ss", &["-tulpn"], "netstat", &["-an"]);
            #[cfg(not(any(target_os = "macos", target_os = "linux")))]
            return run_command("netstat", &["-an"]);
        }
        SystemResource::Uptime => run_command("uptime", &[]),
        SystemResource::Os => run_command("uname", &["-a"]),
    }
}

pub fn handle_query(query: &str) -> Result<()> {
    let q = query.to_lowercase();

    if q.contains("port") {
        if let Some(caps) = PORT_PATTERN.captures(&q) {
            let port = caps.get(1).map(|m| m.as_str()).unwrap_or("0");
            #[cfg(target_os = "macos")]
            return run_command("lsof", &["-i", &format!(":{}", port)]);
            #[cfg(target_os = "linux")]
            return run_command_or_fallback(
                "lsof",
                &["-i", &format!(":{}", port)],
                "ss",
                &["-tlpn", &format!("sport = :{}", port)],
            );
            #[cfg(not(any(target_os = "macos", target_os = "linux")))]
            return run_command("netstat", &["-an"]);
        } else {
            return handle("ports");
        }
    }

    if q.contains("memory") || q.contains("ram") {
        return handle("memory");
    }

    if q.contains("disk") || q.contains("space") {
        return handle("disk");
    }

    if q.contains("cpu") {
        #[cfg(target_os = "macos")]
        return run_command("ps", &["aux", "-r"]);
        #[cfg(target_os = "linux")]
        return run_command("ps", &["aux", "--sort=-%cpu"]);
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        return run_command("ps", &["aux"]);
    }

    Err(AskError::UnknownQuery(query.to_string()))
}

fn run_command(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd).args(args).status()?;

    if !status.success() {
        return Err(AskError::Generic(format!(
            "Command {} exited with status {}",
            cmd,
            status.code().unwrap_or(-1)
        )));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn run_command_or_fallback(
    cmd1: &str,
    args1: &[&str],
    cmd2: &str,
    args2: &[&str],
) -> Result<()> {
    use std::process::Stdio;

    // Try first command
    if Command::new(cmd1)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok()
    {
        run_command(cmd1, args1)
    } else {
        run_command(cmd2, args2)
    }
}

#[cfg(not(target_os = "linux"))]
fn run_command_or_fallback(
    cmd1: &str,
    args1: &[&str],
    _cmd2: &str,
    _args2: &[&str],
) -> Result<()> {
    run_command(cmd1, args1)
}
