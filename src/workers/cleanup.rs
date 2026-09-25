use crate::store::session;
use std::time::Duration;

pub async fn session_cleanup() {
    let mut interval = actix_web::rt::time::interval(Duration::from_mins(30));
    loop {
        interval.tick().await;
        session::delete_expired();
        println!("[CLEANUP] session cleanup")
    }
}
