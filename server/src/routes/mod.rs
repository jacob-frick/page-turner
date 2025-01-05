mod books; 
mod users;
mod user_book_list;
mod auth;

use tide::Response;
use crate::utils::appstate::AppState;

pub fn configure(app: &mut tide::Server<AppState>) {
    app.with(tide::utils::After(|mut res: Response| async {
        res.insert_header("Access-Control-Allow-Origin", "http://localhost:3000");
        res.insert_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS");
        res.insert_header("Access-Control-Allow-Headers", "Content-Type, Authorization");
        res.insert_header("Access-Control-Allow-Credentials", "true");
        Ok(res)
    }));

    // app.at("/protected").with(auth_session).get(protected_route); // Your route handler

    let mut api = tide::Server::with_state(app.state().clone());
    auth::configure(app);
    books::configure(&mut api);
    // users::configure(&mut api);
    // user_book_list::configure(&mut api);

    app.at("/api").nest(api);


}
