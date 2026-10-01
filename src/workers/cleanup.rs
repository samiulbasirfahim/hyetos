use crate::store::session;
use std::time::Duration;

pub async fn session_cleanup() {
    loop {
        actix_web::rt::time::sleep(Duration::from_mins(30)).await;
        session::delete_expired();
        println!("[CLEANUP] session cleanup")
    }
}
