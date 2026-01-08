use crate::error::{AskError, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub api_key: String,
    #[serde(default = "default_mode")]
    pub default_mode: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_true")]
    pub confirm_exec: bool,
    #[serde(default = "default_true")]
    pub color: bool,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

fn default_mode() -> String {
    "ai".to_string()
}

fn default_model() -> String {
    "claude-sonnet-4-20250514".to_string()
}

fn default_true() -> bool {
    true
}

fn default_max_tokens() -> u32 {
    1024
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_key: String::new(),
            default_mode: default_mode(),
            model: default_model(),
            confirm_exec: default_true(),
            color: default_true(),
            max_tokens: default_max_tokens(),
        }
    }
}

impl Config {
    pub fn valid_keys() -> &'static [&'static str] {
        &["api_key", "default_mode", "model", "confirm_exec", "color", "max_tokens"]
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "api_key" => Some(self.api_key.clone()),
            "default_mode" => Some(self.default_mode.clone()),
            "model" => Some(self.model.clone()),
            "confirm_exec" => Some(self.confirm_exec.to_string()),
            "color" => Some(self.color.to_string()),
            "max_tokens" => Some(self.max_tokens.to_string()),
            _ => None,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "api_key" => self.api_key = value.to_string(),
            "default_mode" => self.default_mode = value.to_string(),
            "model" => self.model = value.to_string(),
            "confirm_exec" => {
                self.confirm_exec = matches!(value.to_lowercase().as_str(), "true" | "1" | "yes")
            }
            "color" => {
                self.color = matches!(value.to_lowercase().as_str(), "true" | "1" | "yes")
            }
            "max_tokens" => {
                self.max_tokens = value
                    .parse()
                    .map_err(|_| AskError::Config(format!("Invalid max_tokens value: {}", value)))?
            }
            _ => return Err(AskError::UnknownConfigKey(key.to_string())),
        }
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let mut config = Config::default();

        // Try XDG config path first
        let config_path = get_config_path();
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            config = serde_json::from_str(&content)?;
        } else {
            // Try legacy .askrc
            let legacy_path = dirs::home_dir()
                .map(|h| h.join(".askrc"))
                .unwrap_or_default();
            if legacy_path.exists() {
                let content = fs::read_to_string(&legacy_path)?;
                config = parse_legacy_config(&content, config);
            }
        }

        // Environment overrides
        config.apply_env_overrides();

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    fn apply_env_overrides(&mut self) {
        if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
            self.api_key = key;
        }
        if let Ok(model) = env::var("ASK_MODEL") {
            self.model = model;
        }
        if env::var("ASK_NO_COLOR").is_ok() {
            self.color = false;
        }
        // Also respect standard NO_COLOR
        if env::var("NO_COLOR").is_ok() {
            self.color = false;
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let xdg_config = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::home_dir()
                .map(|h| h.join(".config"))
                .unwrap_or_default()
        });
    xdg_config.join("ask").join("config.json")
}

fn parse_legacy_config(content: &str, mut config: Config) -> Config {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            let _ = config.set(key, value);
        }
    }
    config
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.model, "claude-sonnet-4-20250514");
        assert!(config.color);
        assert_eq!(config.max_tokens, 1024);
    }

    #[test]
    fn test_config_get_set() {
        let mut config = Config::default();
        config.set("max_tokens", "2048").unwrap();
        assert_eq!(config.get("max_tokens"), Some("2048".to_string()));

        config.set("color", "false").unwrap();
        assert!(!config.color);
    }

    #[test]
    fn test_config_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        let mut config = Config::default();
        config.api_key = "test-key".to_string();
        config.max_tokens = 512;

        let content = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&config_path, content).unwrap();

        let loaded: Config = serde_json::from_str(&fs::read_to_string(&config_path).unwrap()).unwrap();
        assert_eq!(loaded.api_key, "test-key");
        assert_eq!(loaded.max_tokens, 512);
    }

    #[test]
    fn test_parse_legacy_config() {
        let content = r#"
api_key = sk-test-123
max_tokens = 2048
color = false
# comment line
"#;
        let config = parse_legacy_config(content, Config::default());
        assert_eq!(config.api_key, "sk-test-123");
        assert_eq!(config.max_tokens, 2048);
        assert!(!config.color);
    }
}
