use crate::config::{get_config_path, Config};
use crate::error::Result;
use colored::Colorize;

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
        config.set(key, value)?;
        config.save()?;
        println!("Set {} = {}", key.green(), value);
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
