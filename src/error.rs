use thiserror::Error;

#[derive(Debug, Error)]
pub enum AskError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("{provider} requires an API key.\nSet it via: ask config api_key=<your-key>\nOr set {env_var} environment variable")]
    MissingApiKey { provider: String, env_var: String },

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("Unknown resource: {0}\nAvailable: disk, memory, cpu, ports, uptime, os")]
    UnknownResource(String),

    #[error("Command not found: {0}")]
    CommandNotFound(String),

    #[error("Unknown config key: {0}")]
    UnknownConfigKey(String),

    #[error("Unknown provider: {0}\nAvailable: anthropic, openai, gemini, ollama, perplexity, groq, mistral, cohere, together")]
    UnknownProvider(String),

    #[error("Custom provider not confirmed. Run 'ask config' to confirm the custom provider configuration.")]
    CustomProviderNotConfirmed,

    #[error("Not sure how to query: {0}")]
    UnknownQuery(String),

    #[error("{0}")]
    Generic(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Request(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, AskError>;
