use dotenvy::dotenv;
use std::sync::OnceLock;

const DEFAULT_PROMPT: &str = "
You are a helpful assistant. Answer the user's question as best as you can. If you don't know the answer, just say that you don't know, don't try to make up an answer.
You mainly manage Events, using google calendar. List events, create events, give reminders, and answer questions about events. You are friendly and concise.
Only answer what was asked. Do not provide additional information or context.
CRITICAL INSTRUCTION: Return strictly plain text. Do NOT use markdown, bolding, a sterisks, or any special formatting.
";

pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub telegram_token: String,
    pub google_web_client_id: String,
    pub google_web_client_secret: String,
    pub google_redirect_uri: String,
    pub public_url: String,
    pub gemini_api_key: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn load() {
        dotenv().ok();

        CONFIG.get_or_init(|| Config {
            database_url: Self::env("DATABASE_URL")
                .parse()
                .expect("DATABASE_URL is missing"),
            telegram_token: Self::env("TELEGRAM_TOKEN")
                .parse()
                .expect("telegram_token is missing"),
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

            public_url: Self::env("PUBLIC_URL")
                .parse()
                .expect("PUBLIC_URL is missing"),

            gemini_api_key: Self::env("GEMINI_API_KEY"),
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
