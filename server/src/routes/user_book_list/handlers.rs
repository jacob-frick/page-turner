
// use tide::{Request, Response, StatusCode};
// use crate::utils::appstate::AppState;
// use crate::models::book_status::get_status_id;
// use crate::models::user_book_list::get_flat_user_book_list;
// use crate::models::book::get_book_by_title;
// use crate::models::user_book_list::insert_user_book_list;

// use serde::{Deserialize};
// use sqlx::Error;
// use crate::models::user::get_user_id_by_username;

// #[derive(Debug, Deserialize)]
// struct UserBookListQueryParams {
//     username: String,
//     title: String,
//     status: String,
// }

// pub async fn add_book_to_user_list(req: Request<AppState>) -> tide::Result {
    
//     let params: UserBookListQueryParams = req.query()?;
//     let pool = &req.state().db_pool;

//     let user_id = match get_user_id_by_username(pool, &params.username).await {
//         Ok(user_id) => user_id,
//         Err(_) => {
//             let mut response = tide::Response::new(StatusCode::NotFound);
//             response.set_body(format!("User '{}' not found", params.username));
//             return Ok(response); 
//         }
//     };

//     let book_id = match get_book_by_title(pool, &params.title).await {
//         Ok(book_id) => book_id,
//         Err(_) => {
//             let mut response = tide::Response::new(StatusCode::NotFound);
//             response.set_body(format!("Book '{}' not found", params.title));
//             return Ok(response); 
//         }
//     };

//     let status_id = match get_status_id(&pool, &params.status).await {
//         Ok(status_id) => status_id,
//         Err(e) => {
//             eprintln!("Failed to parse JSON: {}", e);  // Print out the error
//             return Err(tide::Error::from_str(400, "Invalid JSON").into()); // Return error response
//         }
//     };

//     match insert_user_book_list(pool, &user_id, &book_id, &status_id ).await {
//         Ok(_) => {
//             let mut response = Response::new(StatusCode::Created);
//             response.set_body(format!("Successfully added '{}' to '{}'", params.title, params.username));
//             Ok(response)
//         }
//         Err(Error::Database(db_error)) if db_error.constraint() == Some("user_book_list_user_id_book_id_key") => {
//             let mut response = Response::new(StatusCode::Conflict);
//             response.set_body(format!("Record already exists for '{}' and '{}'", params.title, params.username));
//             Ok(response)
//         }
//         Err(e) => {
//             let mut response = Response::new(StatusCode::InternalServerError);
//             response.set_body(format!("Failed to add book: {}", e));
//             Ok(response)
//         }
//     }

// }

// pub async fn get_book_to_user_list(req: Request<AppState>) -> tide::Result {
//     let username = req.param("username").unwrap_or("world");
//     let pool = &req.state().db_pool;
//     let user_id = match get_user_id_by_username(pool, username).await {
//         Ok(user_id) => user_id,
//         Err(_) => {
//             let mut response = tide::Response::new(StatusCode::NotFound);
//             response.set_body(format!("User '{}' not found", username));
//             return Ok(response); 
//         }
//     };
    
//     let flat_user_book_list = match get_flat_user_book_list(pool, &user_id).await{
//         Ok(flat_user_book_list) => flat_user_book_list,
//         Err(e) => {
//             let mut response = tide::Response::new(StatusCode::NotFound);
//             response.set_body(format!("Empty '{}' not found", e));
//             return Ok(response); 
//         } 
//     };

//     let json_response = serde_json::to_string(&flat_user_book_list)?;
//     let mut response = Response::new(200);
//     response.set_body(json_response);
//     response.insert_header("Content-Type", "application/json");
//     Ok(response)
// }

