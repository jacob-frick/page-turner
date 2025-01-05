
use uuid::Uuid; 

#[derive(serde::Deserialize, serde::Serialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid, 
    pub email: String,
    // pub create_time: Option<String>,
    // pub update_time: Option<String>,
}

pub async fn get_user_id_by_email(pool: &sqlx::PgPool, username: &str) -> tide::Result<Uuid> {
    sqlx::query!(
        r#"
        SELECT id FROM users WHERE email = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await
    .map(|record| record.id)
    .map_err(|_| tide::Error::from_str(tide::StatusCode::NotFound, "User not found"))
}
