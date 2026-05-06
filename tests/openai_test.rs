use httpmock::prelude::*;
use serde_json::json;
use talrune::llm::openai::OpenAiBackend;
use talrune::llm::LlmBackend;

#[tokio::test]
async fn sends_well_formed_request_and_parses_response() {
    let server = MockServer::start();

    let expected_body = json!({
        "model": "gpt-4o-mini",
        "messages": [{"role": "user", "content": "Teste"}],
        "temperature": 0.7
    });

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/chat/completions")
            .header("Authorization", "Bearer testkey")
            .header("Content-Type", "application/json")
            .json_body(expected_body);
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"choices":[{"message":{"content":"Olá"}}]}"#);
    });

    let backend = OpenAiBackend::new(
        server.url("/chat/completions"),
        "gpt-4o-mini".to_string(),
        "testkey".to_string(),
    );

    let result = backend.complete("Teste").await.unwrap();

    assert_eq!(result, "Olá");
    mock.assert();
}

#[tokio::test]
async fn rejects_input_above_token_limit() {
    let server = MockServer::start();
    let backend = OpenAiBackend::new(
        server.url("/chat/completions"),
        "gpt-4o-mini".to_string(),
        "testkey".to_string(),
    );

    let long = "lorem ipsum ".repeat(5000);
    let err = backend.complete(&long).await.unwrap_err();

    assert!(matches!(err, talrune::llm::LlmError::InputTooLong { .. }));
}

#[tokio::test]
async fn maps_401_to_auth_error() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST).path("/chat/completions");
        then.status(401).body(r#"{"error":{"message":"invalid key"}}"#);
    });

    let backend = OpenAiBackend::new(
        server.url("/chat/completions"),
        "gpt-4o-mini".to_string(),
        "bad".to_string(),
    );

    let err = backend.complete("hi").await.unwrap_err();
    assert!(matches!(err, talrune::llm::LlmError::Auth(_)));
}
