use tide::{Response};
use tide::{ Request,Result};
use tide::{ Next};
use tide::StatusCode;
// Your custom Before middleware handler

use std::future::Future;
use std::pin::Pin;
use tide::prelude::*;

/// middleware function to check if user_id exists in session and therefore access to secure
pub fn auth_session<State: Clone + Send + Sync + 'static>(
    req: Request<State>,
    next: Next<'_, State>,
) -> Pin<Box<dyn Future<Output = Result> + Send + '_>> {
    Box::pin(async {

        // check if session and user exist
        let user_id: Option<String> = req.session().get("user_id");

        tide::log::info!("Middleware::user_secure: checking if user exists");

        match user_id {
            Some(_) => Ok(next.run(req).await),
            None => Ok(Response::builder(StatusCode::Unauthorized)
                .content_type("application/json")
                .body(json!({"result": "unauthorized"}))
                .build()
                )
        }
    })
}
