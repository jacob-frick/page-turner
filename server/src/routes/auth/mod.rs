pub mod handlers; 

use tide::Server;
use crate::utils::appstate::AppState;


pub fn configure(app: &mut Server<AppState>) {
    app.at("/auth/login").get(handlers::login);
    app.at("/auth/callback").get(handlers::callback);   
    app.at("/auth/validate_session").get(handlers::validate_session);   
}
