use crate::config::{get_config_path, Config};
use crate::error::Result;
use crate::providers::{is_known_provider_url, ProviderType};
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
            let value = config.get(key);
            let display_value = match key {
                &"api_key" => {
                    if let Some(ref v) = value {
                        if !v.is_empty() {
                            format!("{}...(hidden)", &v[..v.len().min(8)])
                        } else {
                            "(not set)".to_string()
                        }
                    } else {
                        "(not set)".to_string()
                    }
                }
                &"model" | &"api_url" => value.unwrap_or_else(|| "(default)".to_string()),
                _ => value.unwrap_or_default(),
            };
            println!("  {} = {}", key, display_value);
        }

        // Show effective values for provider
        println!();
        println!("Effective settings:");
        println!("  model (effective) = {}", config.effective_model());
        println!("  api_url (effective) = {}", config.effective_api_url());

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

        // Check if this requires security confirmation
        let needs_confirmation = match key {
            "provider" => {
                // Check if this is an unknown provider
                value.parse::<ProviderType>().is_err()
            }
            "api_url" => {
                // Check if this is an unknown URL
                !value.is_empty() && !is_known_provider_url(value)
            }
            _ => false,
        };

        if needs_confirmation {
            if !confirm_custom_provider(key, value)? {
                println!("{}", "Configuration cancelled.".yellow());
                return Ok(());
            }
            // Mark custom provider as confirmed
            config.custom_provider_confirmed = true;
        }

        config.set(key, value)?;
        config.save()?;
        println!(
            "Set {} = {}",
            key.green(),
            if SENSITIVE_KEYS.contains(&key) {
                "(hidden)".to_string()
            } else {
                value.to_string()
            }
        );
        return Ok(());
    }

    // Check if this is a sensitive key without a value - show current and prompt to change
    if SENSITIVE_KEYS.contains(&args) {
        let current = config.get(args).unwrap_or_default();

        if current.is_empty() {
            // No value set, prompt for new value
            let value = prompt_secret(args)?;
            if value.is_empty() {
                println!("{}", "No value entered, config unchanged.".yellow());
                return Ok(());
            }
            config.set(args, &value)?;
            config.save()?;
            println!("Set {} = (hidden)", args.green());
        } else {
            // Value exists, show truncated and ask if user wants to change
            let truncated = format!("{}...", &current[..current.len().min(8)]);
            println!("{} is currently set to: {}", args, truncated);
            print!("Do you want to change it? [y/N]: ");
            io::stdout().flush()?;

            let mut response = String::new();
            io::stdin().read_line(&mut response)?;

            if response.trim().eq_ignore_ascii_case("y") {
                let value = prompt_secret(args)?;
                if value.is_empty() {
                    println!("{}", "No value entered, config unchanged.".yellow());
                    return Ok(());
                }
                config.set(args, &value)?;
                config.save()?;
                println!("Set {} = (hidden)", args.green());
            } else {
                println!("Config unchanged.");
            }
        }
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
    eprintln!(
        "{}",
        "WARNING: API key may be saved in your shell history!"
            .yellow()
            .bold()
    );
    eprintln!();
    eprintln!("To remove it from history:");
    eprintln!(
        "  {}: Edit ~/.bash_history and remove the line",
        "Bash".cyan()
    );
    eprintln!(
        "  {}: Edit ~/.zsh_history and remove the line",
        "Zsh".cyan()
    );
    eprintln!();
    eprintln!(
        "For secure input next time, use: {}",
        "ask config api_key".green()
    );
    eprintln!();
}

/// Confirm custom provider or URL with security warning
fn confirm_custom_provider(key: &str, value: &str) -> Result<bool> {
    eprintln!();
    eprintln!(
        "{}",
        "SECURITY WARNING".red().bold()
    );
    eprintln!();
    eprintln!(
        "You are configuring a custom {} that is not in the trusted allowlist:",
        key
    );
    eprintln!("  {} = {}", key.yellow(), value.yellow());
    eprintln!();
    eprintln!("This configuration will send your queries and API key to this destination.");
    eprintln!("Only proceed if you trust this {} completely.", key);
    eprintln!();
    eprintln!("Known providers: {}", ProviderType::known_providers().join(", "));
    eprintln!();
    print!("Type '{}' to confirm: ", "yes".green());
    io::stdout().flush()?;

    let mut response = String::new();
    io::stdin().read_line(&mut response)?;

    Ok(response.trim().eq_ignore_ascii_case("yes"))
}
