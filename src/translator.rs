use crate::llm::{LlmBackend, LlmError};

pub async fn translate(
    text: &str,
    target_lang: &str,
    backend: &dyn LlmBackend,
) -> Result<String, LlmError> {
    let language_name = language_name(target_lang);
    let prompt = format!(
        "Translate the following text to {language_name}. Return only the translation, no explanations:\n{text}"
    );
    backend.complete(&prompt).await
}

fn language_name(code: &str) -> String {
    match code.to_lowercase().as_str() {
        "pt" => "Portuguese".to_string(),
        "en" => "English".to_string(),
        "es" => "Spanish".to_string(),
        "fr" => "French".to_string(),
        "de" => "German".to_string(),
        "it" => "Italian".to_string(),
        "ja" => "Japanese".to_string(),
        "zh" => "Chinese".to_string(),
        "ru" => "Russian".to_string(),
        "ar" => "Arabic".to_string(),
        other => other.to_string(),
    }
}
