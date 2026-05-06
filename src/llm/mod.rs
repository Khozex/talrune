pub mod ollama;
pub mod openai;

use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("connection failed to {url}: {hint}")]
    Connection { url: String, hint: String },

    #[error("authentication failed: {0}")]
    Auth(String),

    #[error("input is too long ({tokens} tokens, max {max})")]
    InputTooLong { tokens: usize, max: usize },

    #[error("provider error: {0}")]
    Provider(String),

    #[error("failed to decode response: {0}")]
    Decode(String),
}

#[async_trait]
pub trait LlmBackend: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<String, LlmError>;
}

use crate::cli::Provider;
use crate::config::Config;

pub fn build(config: &Config) -> Box<dyn LlmBackend> {
    match config.provider {
        Provider::Ollama => Box::new(ollama::OllamaBackend::new(
            config.base_url.clone(),
            config.model.clone(),
        )),
        Provider::Openai => Box::new(openai::OpenAiBackend::new(
            config.base_url.clone(),
            config.model.clone(),
            config.api_key.clone().expect("api_key required for OpenAI; resolve() must enforce"),
        )),
    }
}
