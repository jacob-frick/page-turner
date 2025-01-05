
use oauth2::TokenResponse;
use tide::{Request, Response};
use oauth2::{AuthorizationCode, Scope, CsrfToken};
use serde::Deserialize;
use crate::utils::appstate::AppState;
use tokio::runtime::Runtime;
use tide::StatusCode;
use serde_json::Value;
use serde::Serialize;
use failure::Fail;
use sqlx::query;
use sqlx::query_as;
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
    pub email: String,
    pub full_name: String,
    pub first_name: String,
    pub last_name: String,
}

pub async fn login(req: Request<AppState>) -> tide::Result {
    let app_state = req.state();
    let oauth_client = app_state.oauth_client.clone();
    let (authorize_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    let authorize_url = authorize_url.to_string();

    let response = tide::Response::builder(302)
        .header("Location", authorize_url)
        .build();

    Ok(response)
}
#[derive(Debug, Deserialize)]
struct OAuthCallbackRequest  {
    code: String,
}

pub async fn callback(mut req: Request<AppState>) -> tide::Result {
    let query: OAuthCallbackRequest = req.query()?;
    let code = AuthorizationCode::new(query.code);
    let token_req = req.state().oauth_client.exchange_code(code);
    let rt = Runtime::new().unwrap();
    let token = rt.block_on(token_req.request_async(oauth2::reqwest::async_http_client)).map_err(Fail::compat)?;
    let access_token = token.access_token();

    let userinfo: Value = surf::get("https://www.googleapis.com/oauth2/v2/userinfo?alt=json")
        .header("Authorization", format!("Bearer {}", access_token.secret()))
        .recv_json()
        .await
        .map_err(|err| tide::Error::from_str(500, format!("Fetching user info failed: {}", err)))?;
    println!("{:?}", userinfo);
    let user_info = UserInfo {
        email: userinfo["email"].as_str().unwrap_or_default().to_string(),
        full_name: userinfo["name"].as_str().unwrap_or_default().to_string(),
        first_name: userinfo["given_name"].as_str().unwrap_or_default().to_string(),
        last_name: userinfo["family_name"].as_str().unwrap_or_default().to_string(),
    };

    let pool = &req.state().db_pool;

    // Step 1: Check if the user exists
    let user = query_as!(
        DbUser,
        "SELECT id, email FROM users WHERE email = $1",
        user_info.email
    )
    .fetch_optional(pool)
    .await?;

    // Step 2: Insert the user if they don't exist
    let user_id = if let Some(existing_user) = user {
        existing_user.id
    } else {
        query!(
            "INSERT INTO users (email, first_name, last_name, full_name) VALUES ($1, $2, $3, $4) RETURNING id",
            user_info.email,
            user_info.first_name,
            user_info.last_name,
            user_info.full_name
        )
        .fetch_one(pool)
        .await?
        .id
    };

    // Step 3: Create a session with the user's ID and email
    let session = req.session_mut();
    session.insert("user_id", user_id)?;
    session.insert("email", user_info.email)?;

    // Redirect the user to the dashboard
    let mut response = Response::new(StatusCode::Found);
    response.insert_header("Location", "http://localhost:3000/dashboard");
    Ok(response)

    // let session = req.session_mut();
    // session.insert("user_email", user_info.email.clone())?;

    // // let redirect_url = "http://localhost:3000/dashboard";

    // let mut response = Response::new(StatusCode::Found);
    // // response.insert_header("Location", redirect_url);
    // Ok(response)
}
#[derive(Debug)]
struct DbUser {
    id: uuid::Uuid,
    email: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
}

pub async fn validate_session(req: Request<AppState>) -> tide::Result {
    let session = req.session();

    // Extract session data
    if let (Some(user_id_str), Some(email)) = (
        session.get::<String>("user_id"),
        session.get::<String>("email"),
    ) {
        // Convert user_id to Uuid
        let user_id = Uuid::parse_str(&user_id_str).map_err(|_| {
            tide::Error::from_str(StatusCode::BadRequest, "Invalid user ID format")
        })?;

        let pool = &req.state().db_pool;

        // Query the database to validate the user
        let user = query_as!(
            User,
            r#"
            SELECT id, email, first_name, last_name, full_name
            FROM users
            WHERE id = $1 AND email = $2
            "#,
            user_id,
            email
        )
        .fetch_optional(pool)
        .await?;

        if let Some(user) = user {
            let user_info = json!({
                "id": user.id,
                "email": user.email,
                "first_name": user.first_name,
                "last_name": user.last_name,
                "full_name": user.full_name,
            });

            return Ok(Response::builder(StatusCode::Ok)
                .body(user_info)
                .content_type(tide::http::mime::JSON)
                .build());
        } else {
            // User not found in the database
            return Ok(Response::builder(StatusCode::Unauthorized)
                .body(json!({"error": "Invalid session"}))
                .content_type(tide::http::mime::JSON)
                .build());
        }
    } else {
        // Missing session data
        Ok(Response::builder(StatusCode::Unauthorized)
            .body(json!({"error": "Unauthorized"}))
            .content_type(tide::http::mime::JSON)
            .build())
    }
}
