use crate::dtos::shortener_url_dto::{ShortenerUrlDTO, ShortenerUrlResponseDTO};
use axum::{Json, extract::State, http::StatusCode};
use rand::{Rng, distr::Alphanumeric, rng};
use sqlx::PgPool;

pub struct ShortenerUrlHandler;

impl ShortenerUrlHandler {
    pub async fn execute(
        db: &PgPool,
        payload: ShortenerUrlDTO,
    ) -> Result<Json<ShortenerUrlResponseDTO>, StatusCode> {
        if payload.url.trim().is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }

        let short_id = Self::generate_unique_short_id(db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Self::insert_url(db, &payload.url, &short_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ShortenerUrlResponseDTO {
            short_url_code: short_id,
        }))
    }

    async fn generate_unique_short_id(db: &PgPool) -> Result<String, sqlx::Error> {
        loop {
            let short_id: String = rng()
                .sample_iter(Alphanumeric)
                .take(8)
                .map(char::from)
                .collect();

            let exists =
                sqlx::query_scalar::<_, Option<i64>>("SELECT 1 FROM url WHERE short = $1 LIMIT 1")
                    .bind(&short_id)
                    .fetch_optional(db)
                    .await?;

            if exists.is_none() {
                return Ok(short_id);
            }
        }
    }

    async fn insert_url(
        db: &PgPool,
        original_url: &str,
        short_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO url (original, short) VALUES ($1, $2)")
            .bind(original_url)
            .bind(short_id)
            .execute(db)
            .await
            .map(|_| ())
    }
}

#[axum::debug_handler]
pub async fn execute(
    State(db): State<PgPool>,
    Json(payload): Json<ShortenerUrlDTO>,
) -> Result<Json<ShortenerUrlResponseDTO>, StatusCode> {
    ShortenerUrlHandler::execute(&db, payload).await
}
