use crate::db::DBPool;
use crate::models::Auth;
use crate::types::platform::Platform;
use crate::types::session::{ExternalSession, ExternalSessionAction};
use crate::utils::random;
use crate::{Config, store};
use chrono::{Duration, Utc};

pub async fn connect(platform: &Platform, pool: &DBPool) -> String {
    let user = Auth::get_user_by_platform_user(pool, platform).await;
    if let Ok(user) = user {
        return format!(
            "You are already connected as {}.\n\n\nIf you want to connect a different account, please disconnect first.",
            user.mail_address
        );
    }

    println!("Creating session for platform: {:?}", platform);
    println!("DEBUG INFO: {:?}", user);

    let public_url = Config::get().public_url.clone();

    let state = random::generate_random_string(24);

    let session = ExternalSession {
        platform: platform.clone(),
        action: ExternalSessionAction::Connect,
        exipres_at: Utc::now() + Duration::minutes(10),
    };

    let url = format!(
        "Use the following link to connect Hyetos with your Telegram account: {}{}?state={}",
        public_url, "/auth/google", state
    );

    if let Err(_) = store::session::add(state, session) {
        return "Failed to create session. Please try again.".to_string();
    }

    url
}
