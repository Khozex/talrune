use async_trait::async_trait;
use serde_json::{json, Value};
use tiktoken_rs::cl100k_base;

use super::{LlmBackend, LlmError};

const MAX_TOKENS: usize = 4096;

pub struct OpenAiBackend {
    client: reqwest::Client,
    url: String,
    model: String,
    api_key: String,
}

impl OpenAiBackend {
    pub fn new(url: String, model: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            url,
            model,
            api_key,
        }
    }
}

#[async_trait]
impl LlmBackend for OpenAiBackend {
    async fn complete(&self, prompt: &str) -> Result<String, LlmError> {
        let bpe = cl100k_base().map_err(|e| LlmError::Provider(format!("tokenizer init: {e}")))?;
        let tokens = bpe.encode_ordinary(prompt);
        if tokens.len() > MAX_TOKENS {
            return Err(LlmError::InputTooLong {
                tokens: tokens.len(),
                max: MAX_TOKENS,
            });
        }

        let body = json!({
            "model": self.model,
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7
        });

        let response = self
            .client
            .post(&self.url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Connection {
                url: self.url.clone(),
                hint: e.to_string(),
            })?;

        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(|e| LlmError::Decode(e.to_string()))?;

        if status == 401 || status == 403 {
            return Err(LlmError::Auth(text));
        }
        if !status.is_success() {
            return Err(LlmError::Provider(format!("HTTP {status}: {text}")));
        }

        let parsed: Value =
            serde_json::from_str(&text).map_err(|e| LlmError::Decode(format!("{e}: {text}")))?;
        parsed["choices"][0]["message"]["content"]
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| LlmError::Decode(format!("no content in response: {text}")))
    }
}
