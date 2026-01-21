use crate::error::{AskError, Result};
use crate::providers::streaming::build_system_prompt;
use crate::providers::{Provider, ProviderConfig};
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
}

#[derive(Deserialize)]
struct StreamResponse {
    message: Option<MessageResponse>,
    done: Option<bool>,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: Option<String>,
}

pub struct OllamaProvider;

#[async_trait]
impl Provider for OllamaProvider {
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
        };

        let response = match client
            .post(&config.api_url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                // Check if it's a connection error (Ollama not running)
                if e.is_connect() {
                    return Err(AskError::Generic(format!(
                        "Could not connect to Ollama at {}. Is Ollama running?\n\
                         Start it with: ollama serve\n\
                         Or install from: https://ollama.ai",
                        config.api_url
                    )));
                }
                return Err(e.into());
            }
        };

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();

            // Check for model not found error
            if status == 404 || body.contains("model") && body.contains("not found") {
                return Err(AskError::Generic(format!(
                    "Model '{}' not found in Ollama.\n\
                     Pull it with: ollama pull {}\n\
                     Or list available models: ollama list",
                    config.model, config.model
                )));
            }

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

            // Ollama uses NDJSON (newline-delimited JSON)
            while let Some(pos) = buffer.find('\n') {
                let line = buffer[..pos].trim().to_string();
                buffer = buffer[pos + 1..].to_string();

                if line.is_empty() {
                    continue;
                }

                if let Ok(response) = serde_json::from_str::<StreamResponse>(&line) {
                    if let Some(message) = response.message {
                        if let Some(content) = message.content {
                            print!("{}", content);
                            io::stdout().flush()?;
                        }
                    }
                    // Check if done
                    if response.done == Some(true) {
                        break;
                    }
                }
            }
        }

        println!(); // Final newline
        Ok(())
    }
}
