mod utils;
mod routes;
mod models;
use routes::configure;


use utils::appstate::get_appstate;
use tide::sessions::{SessionMiddleware, CookieStore};
use tide::Server;


#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let app_state = get_appstate().await?;
    let mut app = Server::with_state(app_state);

    let secret = b"your-secret-key-thisshouldbealongerbetterkey"; // Replace with a secure random key
    let middleware = SessionMiddleware::new(CookieStore::new(), secret)
    .with_cookie_name("custom_session_id");
    
    app.with(middleware);

    configure(&mut app);
    app.listen("localhost:8080").await?;
    Ok(())
}

