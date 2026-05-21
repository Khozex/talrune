use std::env;
use thiserror::Error;

use crate::cli::{Cli, Provider};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("OpenAI API key not set (OPENAI_API_KEY or GPT_TOKEN)")]
    MissingApiKey,
}

#[derive(Debug)]
pub struct Config {
    pub provider: Provider,
    pub model: String,
    pub target_lang: String,
    pub base_url: String,
    pub api_key: Option<String>,
}

impl Config {
    pub fn resolve(cli: Cli) -> Result<Self, ConfigError> {
        let provider = cli.provider.unwrap_or(Provider::Ollama);
        let target_lang = cli.target_lang.unwrap_or_else(|| "pt".to_string());

        let model = cli.model.unwrap_or_else(|| match provider {
            Provider::Ollama => "llama3.2".to_string(),
            Provider::Openai => "gpt-4o-mini".to_string(),
        });

        let base_url = cli.base_url.unwrap_or_else(|| match provider {
            Provider::Ollama => "http://localhost:11434".to_string(),
            Provider::Openai => "https://api.openai.com/v1/chat/completions".to_string(),
        });

        let api_key = match provider {
            Provider::Openai => {
                let key = env::var("OPENAI_API_KEY")
                    .or_else(|_| {
                        env::var("GPT_TOKEN").inspect(|_| {
                            eprintln!(
                                "warning: GPT_TOKEN is deprecated, use OPENAI_API_KEY instead"
                            );
                        })
                    })
                    .map_err(|_| ConfigError::MissingApiKey)?;
                Some(key)
            }
            Provider::Ollama => None,
        };

        Ok(Self {
            provider,
            model,
            target_lang,
            base_url,
            api_key,
        })
    }
}
