use crate::config::Config;
use crate::error::{AskError, Result};
use crate::providers::{create_provider, is_known_provider_url, ProviderType};
use colored::Colorize;

pub async fn handle(query: &str, config: &Config) -> Result<()> {
    let provider_type = config.provider;

    // Check if API key is required and missing
    if provider_type.requires_api_key() && config.api_key.is_empty() {
        let env_var = provider_type.env_var_name();
        println!("{}", format!("{} requires an API key", provider_type).red());
        println!("Set it via: ask config api_key=<your-key>");
        println!("Or set {} environment variable", env_var);
        println!("\nYour query: {}", query);
        println!("\n{}:", "Alternatives".yellow());
        println!("  ask howto <task>      - Get command suggestions");
        println!("  ask explain <cmd>     - Get help for a command");
        println!("  ask system <resource> - System information");
        return Err(AskError::MissingApiKey {
            provider: provider_type.to_string(),
            env_var: env_var.to_string(),
        });
    }

    // Check if custom provider/URL needs confirmation
    let has_custom_url = config
        .api_url
        .as_ref()
        .is_some_and(|url| !is_known_provider_url(url));
    let is_custom_provider = matches!(provider_type, ProviderType::Custom);

    if (has_custom_url || is_custom_provider) && !config.custom_provider_confirmed {
        println!(
            "{}",
            "Custom provider configuration requires confirmation.".red()
        );
        println!("Run 'ask config' and re-enter your custom settings to confirm.");
        return Err(AskError::CustomProviderNotConfirmed);
    }

    let provider = create_provider(provider_type);
    let provider_config = config.provider_config();

    provider.stream_response(&provider_config, query).await
}
