pub mod handlers; 

use tide::Server;
use crate::utils::appstate::AppState;


pub fn configure(app: &mut Server<AppState>) {
    app.at("/books").get(handlers::get_books);
    app.at("/books").post(handlers::create_book);   
}
