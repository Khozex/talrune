use talrune::translator;
use httpmock::prelude::*;
use serde_json::{json};

#[tokio::test]

async fn test_make_request () {
    let server = MockServer::start();

    let request_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": "Teste"}],
        "temperature": 0.7
    });

    let request_to_openai_api = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/chat/completions")
            .header("Authorization", "Bearer teste")
            .json_body(request_body.clone())
            .header("Content-Type", "application/json");
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"choices": [{"message": {"content": "Olá, tudo bem?"}}]}"#);
    });

    let api_url = server.url("/v1/chat/completions");
    let response = translator::make_request_to_gpt(String::from("Teste"), "teste".to_string(), api_url).await.unwrap();

    assert_eq!(response, "Olá, tudo bem?".to_string());

    request_to_openai_api.assert();
}

#[tokio::test]
async fn test_if_message_pass_tokenizer () {

    let server = MockServer::start();

    let text = std::fs::read_to_string("tests/lorem.txt").unwrap();
    let api_url = server.url("/v1/chat/completions");
    let response = translator::make_request_to_gpt(text, "teste".to_string(), api_url).await.unwrap();


    assert_eq!(response, "Input is too long".to_string());
    
}
