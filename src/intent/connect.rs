use crate::db::DBPool;
use crate::models::Auth;
use crate::types::intent_return::{AuthorizedInfo, ConnectIntentReturn, IntentReturn};
use crate::types::platform::Platform;
use crate::types::session::{ExternalSession, ExternalSessionAction};
use crate::utils::random;
use crate::{Config, store};
use chrono::{Duration, Utc};

pub async fn connect(platform: &Platform, pool: &DBPool) -> IntentReturn {
    let user = Auth::get_user_by_platform_user(pool, &platform.get_user()).await;
    if let Ok(user) = user {
        return IntentReturn::Connect(ConnectIntentReturn::AuthorizedEmail(AuthorizedInfo {
            email: user.mail_address,
        }));
    }

    let public_url = Config::get().public_url.clone();

    let state = random::generate_random_string(24);

    let session = ExternalSession {
        platform: platform.get_user(),
        action: ExternalSessionAction::Connect,
        exipres_at: Utc::now() + Duration::minutes(10),
    };

    let platform_name = match platform {
        Platform::Telegram(_) => "Telegram",
        Platform::Discord(_) => "Discord",
    };

    let url = format!("{}{}?state={}", public_url, "/auth/google", state);

    if let Err(_) = store::session::add(state, session) {
        return IntentReturn::IntentHandlerError(
            "Failed to create session. Please try again.".to_string(),
        );
    }

    IntentReturn::Connect(ConnectIntentReturn::ConnectionUrl(
        crate::types::intent_return::ConnectionUrl {
            url,
            connecting_to: platform_name.to_string(),
        },
    ))
}
