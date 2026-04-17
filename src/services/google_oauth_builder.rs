use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub fn build_oauth_client() -> BasicClient {
    let config = crate::Config::get();

    BasicClient::new(
        ClientId::new(config.google_web_client_id.clone()),
        Some(ClientSecret::new(config.google_web_client_secret.clone())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(config.google_redirect_uri.clone()).unwrap())
}
