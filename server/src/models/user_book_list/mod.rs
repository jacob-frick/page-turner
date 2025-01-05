// use serde::{Deserialize, Serialize};
// use sqlx::FromRow;
// use uuid::Uuid;

// #[derive(Serialize, Deserialize, Debug, FromRow)]
// pub struct UserBookList {
//     pub user_id: Uuid,
//     pub book_id: Uuid,
//     pub status_id: Uuid, // FK to the BookStatus table (UUID)
// }

// #[derive(Serialize, Deserialize, Debug, FromRow)]
// pub struct FlatUserBookList {
//     pub user_id: Uuid,
//     pub book_id: Uuid,
//     pub book_title: String,
//     pub book_author: String,
//     pub book_pages: i32,
//     pub book_publish_date: String,
//     pub status_id: Uuid,
// }

// pub async fn get_flat_user_book_list(pool: &sqlx::PgPool, user_id: &Uuid) -> Result<Vec<FlatUserBookList>, sqlx::Error>{
//     sqlx::query_as!(
//         FlatUserBookList,
//         r#"
//         SELECT 
//             ubl.user_id,
//             b.id AS book_id, 
//             b.title AS book_title, 
//             b.author AS book_author, 
//             b.pages AS book_pages, 
//             b.publish_date AS book_publish_date,
//             ubl.status_id
//         FROM user_book_list ubl
//         INNER JOIN books b ON ubl.book_id = b.id
//         WHERE ubl.user_id = $1 AND b.id IS NOT NULL AND ubl.status_id IS NOT NULL
//         "#,
//         user_id,
//     )
//     .fetch_all(pool)
//     .await
// }
// pub async fn insert_user_book_list( pool: &sqlx::PgPool, user_id: &Uuid, book_id: &Uuid, status_id: &Uuid) -> Result<UserBookList, sqlx::Error> {
//     let record = sqlx::query_as!(
//         UserBookList,
//         r#"
//         INSERT INTO user_book_list (user_id, book_id, status_id)
//         VALUES ($1, $2, $3)
//         RETURNING user_id, book_id, status_id
//         "#,
//         user_id,
//         book_id,
//         status_id
//     )
//     .fetch_one(pool)
//     .await?;

//     Ok(record)
// }