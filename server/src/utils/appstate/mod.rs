use sqlx::PgPool;
use anyhow::Result;
use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl};
use oauth2::basic::BasicClient;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub oauth_client: BasicClient,
}

pub async fn get_appstate() -> Result<AppState> {
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    // Set up OAuth client
    let client_id = std::env::var("GOOGLE_CLIENT_ID")?;
    let google_client_id = ClientId::new(client_id.clone());

    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")?;
    let google_client_secret = ClientSecret::new(client_secret.clone());

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;

    let oauth_client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080/auth/callback".to_string())?);


    Ok(AppState { 
        db_pool: pool,
        oauth_client 
    })
}
