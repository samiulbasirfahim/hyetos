mod health;
pub use health::health;

pub async fn check_running() -> &'static str {
    "Server is Running"
}
