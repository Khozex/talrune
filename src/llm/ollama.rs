use async_trait::async_trait;
use serde_json::{json, Value};

use super::{LlmBackend, LlmError};

pub struct OllamaBackend {
    client: reqwest::Client,
    base_url: String,
    model: String,
}

impl OllamaBackend {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            model,
        }
    }
}

#[async_trait]
impl LlmBackend for OllamaBackend {
    async fn complete(&self, prompt: &str) -> Result<String, LlmError> {
        let url = format!("{}/api/chat", self.base_url.trim_end_matches('/'));
        let body = json!({
            "model": self.model,
            "messages": [{"role": "user", "content": prompt}],
            "stream": false
        });

        let response = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Connection {
                url: url.clone(),
                hint: format!(
                    "{e}. Ollama não está rodando? Tente 'ollama serve' ou 'docker compose up ollama'."
                ),
            })?;

        let status = response.status();
        let text = response.text().await
            .map_err(|e| LlmError::Decode(e.to_string()))?;

        if !status.is_success() {
            return Err(LlmError::Provider(format!("HTTP {status}: {text}")));
        }

        let parsed: Value = serde_json::from_str(&text)
            .map_err(|e| LlmError::Decode(format!("{e}: {text}")))?;
        parsed["message"]["content"]
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| LlmError::Decode(format!("no content in response: {text}")))
    }
}
