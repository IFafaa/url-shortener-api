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

#### Como configurar a URL do banco de dados no arquivo `.env`

Adicione a URL de conexão do banco de dados PostgreSQL no seu arquivo `.env` com os valores adequados ao seu ambiente. A URL deve ter a seguinte estrutura:

```env
# URL de conexão do banco de dados PostgreSQL
DATABASE_URL=postgres://<USUARIO>:<SENHA>@<HOST>:<PORTA>/<BANCO_DE_DADOS>
```

### 4. Suba o Docker Compose

Inicie o banco de dados PostgreSQL com o Docker Compose:

```bash
docker-compose up -d
```

### 5. Rode as migrations

Execute as migrations para configurar o banco de dados:

```bash
sqlx migrate run
```

### 6. Execute o projeto

Inicie a aplicação com o comando abaixo. Ele executará o servidor Rust enquanto o banco de dados está rodando no Docker Compose:

```bash
cargo run
```

## Endpoints da API

### 1. Criar URL encurtada

**POST** `/api/shortener`

- **Descrição**: Cria uma URL encurtada a partir de uma URL original.
- **Corpo da requisição**:
  ```json
  {
    "url": "https://exemplo.com"
  }
  ```
- **Resposta de sucesso**:
  - **Status**: `200 OK`
  - **Corpo**:
    ```json
    {
      "shortUrlCode": "abc12345"
    }
    ```
- **Erros possíveis**:
  - `400 BAD REQUEST`: Quando o campo `url` está vazio.
  - `500 INTERNAL SERVER ERROR`: Quando ocorre um erro interno no servidor.

---

### 2. Redirecionar para a URL original

**GET** `/api/short/{id}`

- **Descrição**: Redireciona para a URL original com base no código encurtado.
- **Parâmetros de rota**:
  - `id`: Código da URL encurtada.
- **Resposta de sucesso**:
  - **Status**: `302 FOUND`
  - **Redireciona para**: A URL original.
- **Erros possíveis**:
  - `404 NOT FOUND`: Quando o código encurtado não é encontrado.
  - `500 INTERNAL SERVER ERROR`: Quando ocorre um erro interno no servidor.


## Tecnologias utilizadas

- **Rust**: Linguagem de programação principal.
- **Axum**: Framework web para Rust.
- **Docker Compose**: Gerenciamento do banco de dados PostgreSQL.
- **SQLx**: Biblioteca para interações com o banco de dados.
