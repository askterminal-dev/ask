use crate::error::{AskError, Result};
use crate::providers::streaming::build_system_prompt;
use crate::providers::{Provider, ProviderConfig, ProviderType};
use async_trait::async_trait;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ApiRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct StreamResponse {
    choices: Option<Vec<Choice>>,
}

#[derive(Deserialize)]
struct Choice {
    delta: Option<DeltaContent>,
}

#[derive(Deserialize)]
struct DeltaContent {
    content: Option<String>,
}

/// OpenAI-compatible provider that works with OpenAI, Perplexity, Groq, Mistral, Cohere, Together
pub struct OpenAIProvider;

impl OpenAIProvider {
    pub fn new(_provider_type: ProviderType) -> Self {
        Self
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn stream_response(&self, config: &ProviderConfig, query: &str) -> Result<()> {
        let client = Client::new();
        let system_prompt = build_system_prompt();

        let request = ApiRequest {
            model: config.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                Message {
                    role: "user".to_string(),
                    content: query.to_string(),
                },
            ],
            stream: true,
            max_tokens: config.max_tokens,
        };

        let response = client
            .post(&config.api_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", config.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(AskError::Api {
                status,
                message: body,
            });
        }

        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            // Process complete lines
            while let Some(pos) = buffer.find('\n') {
                let line = buffer[..pos].trim().to_string();
                buffer = buffer[pos + 1..].to_string();

                if let Some(json_str) = line.strip_prefix("data: ") {
                    if json_str == "[DONE]" {
                        continue;
                    }
                    if let Ok(response) = serde_json::from_str::<StreamResponse>(json_str) {
                        if let Some(choices) = response.choices {
                            if let Some(choice) = choices.first() {
                                if let Some(delta) = &choice.delta {
                                    if let Some(content) = &delta.content {
                                        print!("{}", content);
                                        io::stdout().flush()?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!(); // Final newline
        Ok(())
    }
}
