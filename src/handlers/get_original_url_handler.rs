use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
};
use sqlx::PgPool;

pub struct GetOriginalUrlHandler;

impl GetOriginalUrlHandler {
    pub async fn execute(db: &PgPool, short_id: String) -> Result<Redirect, StatusCode> {
        let original_url = Self::fetch_original_url(db, &short_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        match original_url {
            Some(url) => Ok(Redirect::to(&url)),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    async fn fetch_original_url(
        db: &PgPool,
        short_id: &str,
    ) -> Result<Option<String>, sqlx::Error> {
        let result =
            sqlx::query_scalar::<_, Option<String>>("SELECT original FROM url WHERE short = $1")
                .bind(short_id)
                .fetch_optional(db)
                .await?;
        Ok(result.flatten())
    }
}

#[axum::debug_handler]
pub async fn execute(
    State(db): State<PgPool>,
    Path(short_id): Path<String>,
) -> Result<Redirect, StatusCode> {
    GetOriginalUrlHandler::execute(&db, short_id).await
}
