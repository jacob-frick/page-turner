use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum BookStatus {
    ToRead,
    IsReading,
    FinishedReading,
    Favorited,
}

#[derive(FromRow, Deserialize)]
pub struct BookStatusRecord {
    pub id: Uuid
}

pub async fn get_status_id(pool: &sqlx::PgPool, status: &str) -> Result<Uuid, sqlx::Error> {
    let status_record = sqlx::query_as!(
        BookStatusRecord,
        r#"
        SELECT id FROM book_statuses WHERE status = $1
        "#,
        status
    )
    .fetch_one(pool)
    .await?;

    Ok(status_record.id)
}
