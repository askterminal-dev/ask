pub mod anthropic;
pub mod gemini;
pub mod ollama;
pub mod openai;
pub mod streaming;

use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Configuration for a provider instance
pub struct ProviderConfig {
    pub api_key: String,
    pub api_url: String,
    pub model: String,
    pub max_tokens: u32,
}

/// Known LLM provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    #[default]
    Anthropic,
    OpenAI,
    Gemini,
    Ollama,
    Perplexity,
    Groq,
    Mistral,
    Cohere,
    Together,
    Custom,
}

impl ProviderType {
    /// Check if this is a known (trusted) provider
    pub fn is_known(&self) -> bool {
        !matches!(self, ProviderType::Custom)
    }

    /// Get the default API URL for this provider
    pub fn default_api_url(&self) -> &'static str {
        match self {
            ProviderType::Anthropic => "https://api.anthropic.com/v1/messages",
            ProviderType::OpenAI => "https://api.openai.com/v1/chat/completions",
            ProviderType::Gemini => "https://generativelanguage.googleapis.com/v1beta/models",
            ProviderType::Ollama => "http://localhost:11434/api/chat",
            ProviderType::Perplexity => "https://api.perplexity.ai/chat/completions",
            ProviderType::Groq => "https://api.groq.com/openai/v1/chat/completions",
            ProviderType::Mistral => "https://api.mistral.ai/v1/chat/completions",
            ProviderType::Cohere => "https://api.cohere.ai/v1/chat",
            ProviderType::Together => "https://api.together.xyz/v1/chat/completions",
            ProviderType::Custom => "",
        }
    }

    /// Get the default model for this provider
    pub fn default_model(&self) -> &'static str {
        match self {
            ProviderType::Anthropic => "claude-sonnet-4-20250514",
            ProviderType::OpenAI => "gpt-4o",
            ProviderType::Gemini => "gemini-1.5-flash",
            ProviderType::Ollama => "llama3.2",
            ProviderType::Perplexity => "llama-3.1-sonar-small-128k-online",
            ProviderType::Groq => "llama-3.3-70b-versatile",
            ProviderType::Mistral => "mistral-small-latest",
            ProviderType::Cohere => "command-r-plus",
            ProviderType::Together => "meta-llama/Llama-3.3-70B-Instruct-Turbo",
            ProviderType::Custom => "gpt-4o",
        }
    }

    /// Get the environment variable name for this provider's API key
    pub fn env_var_name(&self) -> &'static str {
        match self {
            ProviderType::Anthropic => "ANTHROPIC_API_KEY",
            ProviderType::OpenAI => "OPENAI_API_KEY",
            ProviderType::Gemini => "GEMINI_API_KEY",
            ProviderType::Ollama => "", // No API key required
            ProviderType::Perplexity => "PERPLEXITY_API_KEY",
            ProviderType::Groq => "GROQ_API_KEY",
            ProviderType::Mistral => "MISTRAL_API_KEY",
            ProviderType::Cohere => "COHERE_API_KEY",
            ProviderType::Together => "TOGETHER_API_KEY",
            ProviderType::Custom => "ASK_API_KEY",
        }
    }

    /// Check if this provider requires an API key
    pub fn requires_api_key(&self) -> bool {
        !matches!(self, ProviderType::Ollama)
    }

    /// Get all known provider names for display
    pub fn known_providers() -> &'static [&'static str] {
        &[
            "anthropic",
            "openai",
            "gemini",
            "ollama",
            "perplexity",
            "groq",
            "mistral",
            "cohere",
            "together",
        ]
    }
}

impl fmt::Display for ProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ProviderType::Anthropic => "anthropic",
            ProviderType::OpenAI => "openai",
            ProviderType::Gemini => "gemini",
            ProviderType::Ollama => "ollama",
            ProviderType::Perplexity => "perplexity",
            ProviderType::Groq => "groq",
            ProviderType::Mistral => "mistral",
            ProviderType::Cohere => "cohere",
            ProviderType::Together => "together",
            ProviderType::Custom => "custom",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for ProviderType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "anthropic" => Ok(ProviderType::Anthropic),
            "openai" => Ok(ProviderType::OpenAI),
            "gemini" | "google" => Ok(ProviderType::Gemini),
            "ollama" => Ok(ProviderType::Ollama),
            "perplexity" => Ok(ProviderType::Perplexity),
            "groq" => Ok(ProviderType::Groq),
            "mistral" => Ok(ProviderType::Mistral),
            "cohere" => Ok(ProviderType::Cohere),
            "together" => Ok(ProviderType::Together),
            "custom" => Ok(ProviderType::Custom),
            _ => Err(format!("Unknown provider: {}", s)),
        }
    }
}

/// Trait for LLM providers
#[async_trait]
pub trait Provider: Send + Sync {
    /// Stream a response from the provider
    async fn stream_response(&self, config: &ProviderConfig, query: &str) -> Result<()>;
}

/// Create a provider instance for the given type
pub fn create_provider(provider_type: ProviderType) -> Box<dyn Provider> {
    match provider_type {
        ProviderType::Anthropic => Box::new(anthropic::AnthropicProvider),
        ProviderType::OpenAI => Box::new(openai::OpenAIProvider::new(ProviderType::OpenAI)),
        ProviderType::Gemini => Box::new(gemini::GeminiProvider),
        ProviderType::Ollama => Box::new(ollama::OllamaProvider),
        ProviderType::Perplexity => Box::new(openai::OpenAIProvider::new(ProviderType::Perplexity)),
        ProviderType::Groq => Box::new(openai::OpenAIProvider::new(ProviderType::Groq)),
        ProviderType::Mistral => Box::new(openai::OpenAIProvider::new(ProviderType::Mistral)),
        ProviderType::Cohere => Box::new(openai::OpenAIProvider::new(ProviderType::Cohere)),
        ProviderType::Together => Box::new(openai::OpenAIProvider::new(ProviderType::Together)),
        ProviderType::Custom => Box::new(openai::OpenAIProvider::new(ProviderType::Custom)),
    }
}

/// Check if a URL is a known provider URL
pub fn is_known_provider_url(url: &str) -> bool {
    let known_domains = [
        "api.anthropic.com",
        "api.openai.com",
        "generativelanguage.googleapis.com",
        "localhost",
        "127.0.0.1",
        "api.perplexity.ai",
        "api.groq.com",
        "api.mistral.ai",
        "api.cohere.ai",
        "api.together.xyz",
    ];

    known_domains
        .iter()
        .any(|domain| url.contains(domain))
}
