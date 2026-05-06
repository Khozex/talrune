use talrune::llm::ollama::OllamaBackend;
use talrune::llm::LlmBackend;

/// Smoke test que exige Ollama rodando localmente com llama3.2 baixado.
/// Rode com: `cargo test -- --ignored`
#[tokio::test]
#[ignore]
async fn translates_with_real_ollama() {
    let backend = OllamaBackend::new(
        "http://localhost:11434".to_string(),
        "llama3.2".to_string(),
    );

    let result = backend
        .complete("Translate to Portuguese, only the translation: Hello")
        .await
        .expect("Ollama should respond");

    assert!(!result.trim().is_empty(), "translation should not be empty");
}
