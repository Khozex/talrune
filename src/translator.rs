use serde_json::{json, Value};
use std::error::Error;
use tiktoken_rs::r50k_base;
use reqwest;

pub async fn make_request_to_gpt(input: String, token: String) -> Result<String, Box<dyn Error>> {
    let url = "https://api.openai.com/v1/chat/completions";
    let bpe = r50k_base().unwrap();
    let input = bpe.encode_with_special_tokens(&input as &str);
    if input.len() > 4096 {
        panic!("Input is too long");
    }
    let client = reqwest::Client::new();

    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": input}],
        "temperature": 0.7
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;
    let response_json: Value = serde_json::from_str(&response_text)?;

    if let Some(message_content) = response_json["choices"][0]["message"]["content"].as_str() {
        print!("Message translated: {}", message_content);
        Ok(message_content.to_string())
    } else {
        Err("Failed to extract message content".into())
    }
}
