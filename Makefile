.DEFAULT_GOAL := help

help:  ## Lista todos os targets disponíveis
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN{FS=":.*?## "}{printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

build:  ## Build local (cargo release)
	cargo build --release

install:  ## Instala o binário no PATH (cargo install)
	cargo install --path .

test:  ## Roda testes unitários (mocks)
	cargo test

test-integration:  ## Roda smoke tests (requer Ollama rodando)
	cargo test -- --ignored

up:  ## Sobe Ollama via docker compose
	docker compose up -d ollama

down:  ## Para todos os serviços do compose
	docker compose down

pull-model:  ## Baixa o modelo padrão (llama3.2) no container Ollama
	docker compose exec ollama ollama pull llama3.2

run:  ## Roda o talrune via compose (lê stdin)
	docker compose run --rm talrune

clean:  ## Limpa builds locais
	cargo clean

.PHONY: help build install test test-integration up down pull-model run clean
