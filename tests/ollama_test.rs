use httpmock::prelude::*;
use serde_json::json;
use talrune::llm::ollama::OllamaBackend;
use talrune::llm::LlmBackend;

#[tokio::test]
async fn sends_well_formed_request_and_parses_response() {
    let server = MockServer::start();

    let expected_body = json!({
        "model": "llama3.2",
        "messages": [{"role": "user", "content": "Hello"}],
        "stream": false
    });

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/chat")
            .header("Content-Type", "application/json")
            .json_body(expected_body);
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"message":{"role":"assistant","content":"Olá"}}"#);
    });

    let backend = OllamaBackend::new(server.base_url(), "llama3.2".to_string());

    let result = backend.complete("Hello").await.unwrap();

    assert_eq!(result, "Olá");
    mock.assert();
}

#[tokio::test]
async fn maps_server_error_to_provider_error() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST).path("/api/chat");
        then.status(500).body("model not found");
    });

    let backend = OllamaBackend::new(server.base_url(), "nope".to_string());
    let err = backend.complete("hi").await.unwrap_err();
    assert!(matches!(err, talrune::llm::LlmError::Provider(_)));
}

#[tokio::test]
async fn maps_connection_refused_to_connection_error() {
    let backend = OllamaBackend::new("http://127.0.0.1:1".to_string(), "any".to_string());
    let err = backend.complete("hi").await.unwrap_err();
    match err {
        talrune::llm::LlmError::Connection { hint, .. } => {
            assert!(
                hint.contains("Ollama"),
                "hint should mention Ollama: {hint}"
            );
        }
        other => panic!("expected Connection error, got {other:?}"),
    }
}
