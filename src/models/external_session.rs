use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct ExternalSession {
    pub id: i64,
}

