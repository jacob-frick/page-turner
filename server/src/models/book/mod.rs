
use uuid::Uuid; 

#[derive(serde::Deserialize, serde::Serialize, Debug, sqlx::FromRow)]
pub struct Book {
    pub id: Uuid, 
    pub title: String,
    pub author: String,
    pub pages: i32,
    pub publish_date: String, // Format: YYYY-MM-DD
    // pub create_time: Option<String>,
    // pub update_time: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, sqlx::FromRow)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub pages: i32,
    pub publish_date: String, 
}


pub async fn get_book_by_title(pool: &sqlx::PgPool, title: &str) -> tide::Result<Uuid> {
    let record = sqlx::query!(
        r#"
        SELECT id FROM books WHERE title = $1
        "#,
        title
    )
    .fetch_one(pool)
    .await
    .map_err(|_| tide::Error::from_str(tide::StatusCode::NotFound, "Book not found"))?;

    Ok(record.id)
}