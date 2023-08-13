use serde_json::{json, Value};
use std::env;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() {
    let variable = "GPT_TOKEN";
    let token = env::var(variable).expect("Expected a token in the environment");
    let pre_input = "Traduza para o português:";
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = format!("{} {}", pre_input, input);
    make_request_to_GPT(input, token).await.unwrap();
}


async fn make_request_to_GPT(input: String, token: String) -> Result<String, Box<dyn Error>> {
    let url = "https://api.openai.com/v1/chat/completions";
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
        print!("Message translated: {} ", message_content);
        Ok(message_content.to_string())
    } else {
        Err("Failed to extract message content".into())
    }
}
