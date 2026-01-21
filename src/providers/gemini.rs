use crate::error::{AskError, Result};
use crate::providers::streaming::build_system_prompt;
use crate::providers::{Provider, ProviderConfig};
use async_trait::async_trait;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
    role: Option<String>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
}

#[derive(Serialize)]
struct SystemInstruction {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct ApiRequest {
    contents: Vec<Content>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
    #[serde(rename = "systemInstruction")]
    system_instruction: SystemInstruction,
}

#[derive(Deserialize)]
struct StreamResponse {
    candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Option<CandidateContent>,
}

#[derive(Deserialize)]
struct CandidateContent {
    parts: Option<Vec<ContentPart>>,
}

#[derive(Deserialize)]
struct ContentPart {
    text: Option<String>,
}

pub struct GeminiProvider;

#[async_trait]
impl Provider for GeminiProvider {
    async fn stream_response(&self, config: &ProviderConfig, query: &str) -> Result<()> {
        let client = Client::new();
        let system_prompt = build_system_prompt();

        let request = ApiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: query.to_string(),
                }],
                role: Some("user".to_string()),
            }],
            generation_config: GenerationConfig {
                max_output_tokens: config.max_tokens,
            },
            system_instruction: SystemInstruction {
                parts: vec![Part {
                    text: system_prompt,
                }],
            },
        };

        // Gemini uses URL-based authentication
        // Format: {base_url}/{model}:streamGenerateContent?key={api_key}
        let url = format!(
            "{}/{}:streamGenerateContent?key={}&alt=sse",
            config.api_url, config.model, config.api_key
        );

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
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

            // Process complete lines - Gemini uses SSE with "data: " prefix
            while let Some(pos) = buffer.find('\n') {
                let line = buffer[..pos].trim().to_string();
                buffer = buffer[pos + 1..].to_string();

                if let Some(json_str) = line.strip_prefix("data: ") {
                    if let Ok(response) = serde_json::from_str::<StreamResponse>(json_str) {
                        if let Some(candidates) = response.candidates {
                            if let Some(candidate) = candidates.first() {
                                if let Some(content) = &candidate.content {
                                    if let Some(parts) = &content.parts {
                                        for part in parts {
                                            if let Some(text) = &part.text {
                                                print!("{}", text);
                                                io::stdout().flush()?;
                                            }
                                        }
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
