use dotenvy::dotenv;
use std::sync::OnceLock;

pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub google_web_client_id: String,
    pub google_web_client_secret: String,
    pub google_redirect_uri: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn load() {
        dotenv().ok();

        CONFIG.get_or_init(|| Config {
            database_url: Self::env("DATABASE_URL"),
            port: Self::env("PORT")
                .parse()
                .expect("PORT must be a valid number"),
            google_web_client_id: Self::env("GOOGLE_WEB_CLIENT_ID")
                .parse()
                .expect("GOOGLE_WEB_CLIENT_ID is missing"),
            google_web_client_secret: Self::env("GOOGLE_WEB_CLIENT_SECRET")
                .parse()
                .expect("GOOGLE_WEB_CLIENT_SECRET is missing"),
            google_redirect_uri: Self::env("GOOGLE_REDIRECT_URI")
                .parse()
                .expect("GOOGLE_REDIRECT_URI is missing"),
        });
    }

    fn env(key: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| panic!("Environment variable {} not found", key))
    }

    pub fn get() -> &'static Self {
        CONFIG
            .get()
            .expect("Config not loaded. Call Config::load() before accessing the configuration.")
    }
}
