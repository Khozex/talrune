use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Provider {
    Ollama,
    Openai,
}

#[derive(Debug, Parser)]
#[command(
    name = "talrune",
    version,
    about = "Translate text via local or cloud LLMs"
)]
pub struct Cli {
    /// LLM provider
    #[arg(short, long, env = "TALRUNE_PROVIDER", value_enum)]
    pub provider: Option<Provider>,

    /// Model name (depends on provider)
    #[arg(short, long, env = "TALRUNE_MODEL")]
    pub model: Option<String>,

    /// ISO code of target language (pt, en, es, fr, ...)
    #[arg(short, long, env = "TALRUNE_TARGET_LANG")]
    pub target_lang: Option<String>,

    /// Override provider base URL
    #[arg(long, env = "TALRUNE_BASE_URL")]
    pub base_url: Option<String>,
}
