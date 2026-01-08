use thiserror::Error;

#[derive(Debug, Error)]
pub enum AskError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("AI mode requires an API key.\nSet it via: ask config api_key=sk-ant-...\nOr set ANTHROPIC_API_KEY environment variable")]
    MissingApiKey,

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("Unknown resource: {0}\nAvailable: disk, memory, cpu, ports, uptime, os")]
    UnknownResource(String),

    #[error("Command not found: {0}")]
    CommandNotFound(String),

    #[error("Unknown config key: {0}")]
    UnknownConfigKey(String),

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
