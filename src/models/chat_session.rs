// Forgive me, database gods.
// I needed quick dopamine,
// so I'm dumping pure JSON here
// in this unholy way 
// instead of building actual tables.
// TODO: Fix this properly,
// with proper database model

use crate::db::DBPool;
use crate::types::context::UserSession;

pub struct UserSessionModel;

impl UserSessionModel {
    pub async fn save(
        db: &DBPool,
        user_id: &str,
        session: &UserSession,
    ) -> Result<(), sqlx::Error> {
        let session_data =
            serde_json::to_value(session).map_err(|err| sqlx::Error::Encode(Box::new(err)))?;

        sqlx::query!(
            r#"
        INSERT INTO user_sessions (user_id, session_data, updated_at)
        VALUES ($1, $2, NOW())
        ON CONFLICT (user_id)
        DO UPDATE SET
            session_data = EXCLUDED.session_data,
            updated_at = NOW()
        "#,
            user_id,
            session_data as serde_json::Value
        )
        .execute(db)
        .await?;

        Ok(())
    }
    pub async fn get(db: &DBPool, user_id: &str) -> Result<UserSession, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT session_data AS "session_data: Json<Value>"
            FROM user_sessions
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(db)
        .await?;

        let Some(row) = row else {
            return Err(sqlx::Error::RowNotFound);
        };

        let session: UserSession = serde_json::from_value(row.session_data.0)
            .map_err(|err| sqlx::Error::Decode(Box::new(err)))?;

        Ok(session)
    }

    pub async fn delete(db: &DBPool, user_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM user_sessions
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
