use serde_json::{json, Value};
use std::env;
use std::io;
mod translator;



#[tokio::main]
pub async fn main() {
    let variable = "GPT_TOKEN";
    let token = env::var(variable).expect("Expected a token in the environment");
    let pre_input = "Traduza para o português:";
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input == "" || input == "\n" || input == "\r\n" {
        panic!("Input is empty");
    }
    input = format!("{} {}", pre_input, input);
    translator::make_request_to_gpt(input, token).await.unwrap();
}



