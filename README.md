# talrune

CLI Rust de tradução via LLM, **local-first**: usa Ollama por padrão e
suporta OpenAI como alternativa.

> **Talrune** — *Tal* significa "fala" em dinamarquês/norueguês,
> *Rune* remete à escrita rúnica nórdica.

## Quick start (Docker compose — caminho mais fácil)

```sh
git clone https://github.com/khozex/talrune.git
cd talrune
docker compose up -d ollama
make pull-model               # baixa llama3.2 (~2GB)
echo "Hello, world" | docker compose run --rm talrune
```

## Instalação nativa

Pré-requisitos: [Rust](https://www.rust-lang.org/tools/install) e
[Ollama](https://ollama.com/) instalados.

```sh
ollama pull llama3.2
cargo install --path .
echo "Hello, world" | talrune
```

## Uso

```sh
echo "Hello, world" | talrune                      # default: ollama, llama3.2, pt
echo "Olá, mundo"   | talrune --target-lang en     # traduz pra inglês
echo "Hello"        | talrune --model qwen2.5      # outro modelo do Ollama
echo "Hello"        | talrune --provider openai    # usa OpenAI (precisa OPENAI_API_KEY)
talrune --help                                     # todas as flags
```

## Provider OpenAI

```sh
export OPENAI_API_KEY=sk-...
echo "Hello" | talrune --provider openai
```

`GPT_TOKEN` ainda funciona como fallback, mas está deprecated.

## Configuração

Toda flag tem env var equivalente. Ordem de precedência: **flag > env > default**.

| Flag             | Env                    | Default                                                   |
|------------------|------------------------|-----------------------------------------------------------|
| `--provider`     | `TALRUNE_PROVIDER`     | `ollama`                                                  |
| `--model`        | `TALRUNE_MODEL`        | `llama3.2` (ollama) / `gpt-4o-mini` (openai)              |
| `--target-lang`  | `TALRUNE_TARGET_LANG`  | `pt`                                                      |
| `--base-url`     | `TALRUNE_BASE_URL`     | `http://localhost:11434` / endpoint OpenAI oficial        |
| (sem flag)       | `OPENAI_API_KEY`       | obrigatório se `--provider openai`                        |

## Desenvolvimento

```sh
make test               # mocks (rápido)
make test-integration   # smoke real, requer Ollama rodando
```

## Licença

MIT.
