# URL Shortener API

Este é um projeto de uma API para encurtamento de URLs, desenvolvido em **Rust** utilizando o framework **Axum**. O projeto utiliza **PostgreSQL** como banco de dados, gerenciado via **Docker Compose**, e **SQLx** para interações com o banco.

## Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/)

## Como rodar o projeto

Siga os passos abaixo para configurar e executar o projeto:

### 1. Clone o repositório

```bash
git clone https://github.com/IFafaa/url-shortener-api
```

### 2. Acesse o diretório do projeto

```bash
cd url-shortener-api
```

### 3. Configure o arquivo `.env`

Crie um arquivo `.env` baseado no `.env.example`:

```bash
cp .env.example .env
```

Edite o arquivo `.env` conforme necessário.

### 4. Suba o Docker Compose

Inicie o banco de dados PostgreSQL com o Docker Compose:

```bash
docker-compose up -d
```

### 5. Rode as migrations

Em um outro terminal, execute as migrations para configurar o banco de dados:

```bash
sqlx migrate run
```

### 6. Execute o projeto

Agora inicie a aplicação com o comando abaixo. Ele executará o servidor Rust enquanto o banco de dados está rodando no Docker Compose:

```bash
cargo run
```

## Tecnologias utilizadas

- **Rust**: Linguagem de programação principal.
- **Axum**: Framework web para Rust.
- **Docker Compose**: Gerenciamento do banco de dados PostgreSQL.
- **SQLx**: Biblioteca para interações com o banco de dados.
