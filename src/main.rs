
use std::env;
use std::io;
pub mod translator;



#[tokio::main]
pub async fn main() {
    let variable = "GPT_TOKEN";
    let token = env::var(variable).expect("Expected a token in the environment");
    let pre_input = "Traduza para o português:";
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input == "" || input == "\n" || input == "\r\n" {
        print!("Input is empty");
        return;
    }
    input = format!("{} {}", pre_input, input);
    let url = "https://api.openai.com/v1/chat/completions".to_string();
    translator::make_request_to_gpt(input, token, url).await.unwrap();
}



