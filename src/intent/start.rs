use crate::db::DBPool;
use crate::models::Auth;
use crate::types::platform::Platform;

pub async fn start(pool: &DBPool, platform: &Platform) -> String {
    let user = Auth::get_user_by_platform_user(pool, platform).await;

    match user {
        Ok(user) => format!(
            "Welcome back, {}! You are already connected.\n\nUse /help to see available commands.",
            user.mail_address
        ),
        Err(_) => "Welcome to Hyetos! Use /connect to connect your account.".to_string(),
    }
}
