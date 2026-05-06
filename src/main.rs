use std::io::{self, Read};
use std::process::ExitCode;

use clap::Parser;
use talrune::cli::Cli;
use talrune::config::Config;
use talrune::llm;
use talrune::translator;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    let config = match Config::resolve(cli) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("config error: {e}");
            return ExitCode::from(2);
        }
    };

    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("failed to read stdin: {e}");
        return ExitCode::from(1);
    }

    let trimmed = input.trim();
    if trimmed.is_empty() {
        eprintln!("input is empty");
        return ExitCode::from(1);
    }

    let backend = llm::build(&config);

    match translator::translate(trimmed, &config.target_lang, backend.as_ref()).await {
        Ok(translation) => {
            println!("{translation}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}
