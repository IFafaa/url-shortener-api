# URL Shortener API

This is a project for a URL shortening API, developed in **Rust** using the **Axum** framework. The project uses **PostgreSQL** as the database, managed via **Docker Compose**, and **SQLx** for database interactions.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/)

## How to run the project

Follow the steps below to set up and run the project:

### 1. Clone the repository

```bash
git clone https://github.com/IFafaa/url-shortener-api
```

### 2. Navigate to the project directory

```bash
cd url-shortener-api
```

### 3. Configure the `.env` file

Create a `.env` file based on `.env.example`:

```bash
cp .env.example .env
```

#### How to configure the database URL in the `.env` file

Add the PostgreSQL database connection URL to your `.env` file with the appropriate values for your environment. The URL should have the following structure:

```env
# PostgreSQL database connection URL
DATABASE_URL=postgres://<USER>:<PASSWORD>@<HOST>:<PORT>/<DATABASE>
```

### 4. Start Docker Compose

Start the PostgreSQL database using Docker Compose:

```bash
docker-compose up -d
```

### 5. Run the migrations

Run the migrations to set up the database:

```bash
sqlx migrate run
```

### 6. Run the project

Start the application with the command below. It will run the Rust server while the database is running in Docker Compose:

```bash
cargo run
```

## API Endpoints

### 1. Create a shortened URL

**POST** `/api/shortener`

- **Description**: Creates a shortened URL from an original URL.
- **Request body**:
    ```json
    {
        "url": "https://example.com"
    }
    ```
- **Success response**:
    - **Status**: `200 OK`
    - **Body**:
        ```json
        {
            "shortUrlCode": "abc12345"
        }
        ```
- **Possible errors**:
    - `400 BAD REQUEST`: When the `url` field is empty.
    - `500 INTERNAL SERVER ERROR`: When an internal server error occurs.

---

### 2. Redirect to the original URL

**GET** `/api/short/{id}`

- **Description**: Redirects to the original URL based on the shortened code.
- **Route parameters**:
    - `id`: Shortened URL code.
- **Success response**:
    - **Status**: `302 FOUND`
    - **Redirects to**: The original URL.
- **Possible errors**:
    - `404 NOT FOUND`: When the shortened code is not found.
    - `500 INTERNAL SERVER ERROR`: When an internal server error occurs.
