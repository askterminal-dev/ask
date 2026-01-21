use crate::config::{get_config_path, Config};
use crate::error::Result;
use colored::Colorize;
use std::io::{self, Write};

/// Sensitive config keys that should not be passed on the command line
const SENSITIVE_KEYS: &[&str] = &["api_key"];

pub fn handle(args: &str, config: &mut Config) -> Result<()> {
    let args = args.trim();

    if args.is_empty() || args == "show" {
        println!("Current configuration:");
        println!("  Config file: {}", get_config_path().display());
        println!();
        for key in Config::valid_keys() {
            let value = config.get(key).unwrap_or_default();
            let display_value = if key == &"api_key" && !value.is_empty() {
                format!("{}...(hidden)", &value[..value.len().min(8)])
            } else {
                value
            };
            println!("  {} = {}", key, display_value);
        }
        return Ok(());
    }

    if args == "path" {
        println!("{}", get_config_path().display());
        return Ok(());
    }

    if args == "init" {
        config.save()?;
        println!("Config saved to {}", get_config_path().display());
        return Ok(());
    }

    if let Some((key, value)) = args.split_once('=') {
        let key = key.trim();
        let value = value.trim();

        // Warn about sensitive values in shell history
        if SENSITIVE_KEYS.contains(&key) {
            print_history_warning();
        }

        config.set(key, value)?;
        config.save()?;
        println!("Set {} = {}", key.green(), if SENSITIVE_KEYS.contains(&key) { "(hidden)".to_string() } else { value.to_string() });
        return Ok(());
    }

    // Check if this is a sensitive key without a value - prompt securely
    if SENSITIVE_KEYS.contains(&args) {
        let value = prompt_secret(args)?;
        if value.is_empty() {
            println!("{}", "No value entered, config unchanged.".yellow());
            return Ok(());
        }
        config.set(args, &value)?;
        config.save()?;
        println!("Set {} = (hidden)", args.green());
        return Ok(());
    }

    // Get single key
    if let Some(value) = config.get(args) {
        println!("{}", value);
        Ok(())
    } else {
        println!("{}: {}", "Unknown config key".red(), args);
        println!("Available keys: {}", Config::valid_keys().join(", "));
        std::process::exit(1);
    }
}

/// Prompt for a secret value without echoing to terminal
fn prompt_secret(key: &str) -> Result<String> {
    print!("Enter {}: ", key);
    io::stdout().flush()?;
    let value = rpassword::read_password()?;
    Ok(value)
}

/// Print warning about sensitive data in shell history
fn print_history_warning() {
    eprintln!();
    eprintln!("{}", "WARNING: API key may be saved in your shell history!".yellow().bold());
    eprintln!();
    eprintln!("To remove it from history:");
    eprintln!("  {}: Edit ~/.bash_history and remove the line", "Bash".cyan());
    eprintln!("  {}: Edit ~/.zsh_history and remove the line", "Zsh".cyan());
    eprintln!();
    eprintln!("For secure input next time, use: {}", "ask config api_key".green());
    eprintln!();
}
