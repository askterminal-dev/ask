use crate::api;
use crate::config::Config;
use crate::error::{AskError, Result};
use colored::Colorize;

pub async fn handle(query: &str, config: &Config) -> Result<()> {
    if config.api_key.is_empty() {
        println!("{}", "AI mode requires an API key".red());
        println!("Set it via: ask config api_key=sk-ant-...");
        println!("Or set ANTHROPIC_API_KEY environment variable");
        println!("\nYour query: {}", query);
        println!("\n{}:", "Alternatives".yellow());
        println!("  ask howto <task>      - Get command suggestions");
        println!("  ask explain <cmd>     - Get help for a command");
        println!("  ask system <resource> - System information");
        return Err(AskError::MissingApiKey);
    }

    api::stream_response(config, query).await
}
