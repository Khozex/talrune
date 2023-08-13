# Translate CLI

This is a command-line interface (CLI) tool that translates text from English to Portuguese using the OpenAI GPT-3.5 Turbo model.

## Installation

Before using this CLI, you need to set up an OpenAI API key. Make sure you have [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your system.

1. Clone the repository:

   ```sh
   git clone https://github.com/khozex/translate-cli.git
   cd translate-cli
    ```
2. Set your OpenAI API key as an environment variable:

    ```
    export GPT_TOKEN=your-api-key
    ```
3. Build the CLI:

    ```
    cargo build --release
    ```

## Usage
Once the CLI is running, you can enter English text that you want to translate to Portuguese. The CLI will make use of the OpenAI API to provide you with the translation.

Simply follow the prompts to provide your input.

Example:

```
echo "Hello, my name is John." | ./target/release/translate-cli
```

Output:

```
Olá, meu nome é John.
```

## License

Distributed under the MIT License. 




