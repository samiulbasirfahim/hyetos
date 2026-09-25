use crate::db::DBPool;
use crate::models::Auth;
use crate::types::platform::Platform;

pub async fn start(pool: &DBPool, platform: &Platform) -> String {
    let user_result = Auth::get_user_by_platform_user(pool, &platform.get_user()).await;
    if let Ok(user) = user_result {
        return format!("Welcome back! Your email is: {}", user.mail_address).to_string();
    } else {
        return "Error checking user connection status.".to_string();
    }
}
