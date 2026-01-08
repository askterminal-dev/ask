use crate::config::Config;
use crate::error::{AskError, Result};
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const API_VERSION: &str = "2023-06-01";

#[derive(Serialize)]
struct Message {
    role: &'static str,
    content: String,
}

#[derive(Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    stream: bool,
    system: String,
}

#[derive(Deserialize)]
struct SseEvent {
    #[serde(rename = "type")]
    event_type: String,
    delta: Option<Delta>,
}

#[derive(Deserialize)]
struct Delta {
    #[serde(rename = "type")]
    delta_type: Option<String>,
    text: Option<String>,
}

pub async fn stream_response(config: &Config, query: &str) -> Result<()> {
    if config.api_key.is_empty() {
        return Err(AskError::MissingApiKey);
    }

    let client = Client::new();
    let system_prompt = format!(
        "You are a helpful assistant. Answer questions directly and concisely. \
         Do not mention what you are designed for or add unnecessary caveats about \
         the type of questions you can answer. The user is on {} ({}).",
        std::env::consts::OS,
        std::env::consts::ARCH
    );

    let request = ApiRequest {
        model: config.model.clone(),
        max_tokens: config.max_tokens,
        messages: vec![Message {
            role: "user",
            content: query.to_string(),
        }],
        stream: true,
        system: system_prompt,
    };

    let response = client
        .post(API_URL)
        .header("Content-Type", "application/json")
        .header("x-api-key", &config.api_key)
        .header("anthropic-version", API_VERSION)
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

            if line.starts_with("data: ") {
                let json_str = &line[6..];
                if json_str == "[DONE]" {
                    continue;
                }
                if let Ok(event) = serde_json::from_str::<SseEvent>(json_str) {
                    if event.event_type == "content_block_delta" {
                        if let Some(delta) = event.delta {
                            if delta.delta_type.as_deref() == Some("text_delta") {
                                if let Some(text) = delta.text {
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

    println!(); // Final newline
    Ok(())
}
