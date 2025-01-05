use tide::{Request, Response, StatusCode};
use crate::utils::appstate::AppState;
use crate::models::book::Book;
use crate::models::book::CreateBook;


pub async fn get_books(req: Request<AppState>) -> tide::Result {
    let app_state = req.state();
    let pool = &app_state.db_pool;
    let books = sqlx::query_as!(
        Book,
        "SELECT id, title, author, pages, publish_date FROM books"
    )
    .fetch_all(pool)
    .await?;

    let json_response = serde_json::to_string(&books)?;
    let mut response = Response::new(200);
    response.set_body(json_response);
    response.insert_header("Content-Type", "application/json");
    Ok(response)
}

pub async fn create_book(mut req: Request<AppState>) -> tide::Result {
    let new_book: CreateBook = req.body_json().await?;
    let app_state = req.state();
    let pool = &app_state.db_pool;

    let result = sqlx::query!(
        r#"
        INSERT INTO books (title, author, pages, publish_date)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, author, pages, publish_date
        "#,
        new_book.title,
        new_book.author,
        new_book.pages,
        new_book.publish_date
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(book) => {
            let created_book = Book {
                id: book.id,
                title: book.title,
                author: book.author,
                pages: book.pages,
                publish_date: book.publish_date,
            };

            let json_response = serde_json::to_string(&created_book)?;
            let mut response = Response::new(StatusCode::Created);
            response.set_body(json_response);
            response.insert_header("Content-Type", "application/json");
            Ok(response)
        }
        Err(e) => {
            let mut response = Response::new(StatusCode::InternalServerError);
            response.set_body(format!("Error creating book: {}", e));
            response.insert_header("Content-Type", "text/plain");
            Ok(response)
        }
    }
}