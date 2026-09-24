use crate::db::DBPool;
use actix_web::error::ErrorGone;
use chrono::{DateTime, Utc};
use sqlx::types::uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct Auth {
    pub id: uuid::Uuid,
    pub users: Vec<String>,
    pub google_refresh_token: String,
    pub mail_address: String,
    pub scopes: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl Auth {
    pub async fn insert(
        db: &DBPool,
        initial_user: &String,
        refresh_token: &str,
        mail_address: String,
        scopes: Vec<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO auths (
                users, google_refresh_token, mail_address, scopes
            )
            VALUES (ARRAY[$1], $2, $3, $4)
            "#,
            initial_user,
            refresh_token,
            mail_address,
            &scopes
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn get_user_by_platform_user(
        db: &DBPool,
        platform_user: &str,
    ) -> Result<Auth, sqlx::Error> {
        let row = sqlx::query_as!(
            Auth,
            r#"
            SELECT 
                id, users, google_refresh_token, mail_address, scopes, created_at
            FROM auths
            WHERE $1 = ANY(users)
            "#,
            platform_user
        )
        .fetch_optional(db)
        .await?;

        if let Some(auth) = row {
            Ok(auth)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    pub async fn get_users_by_mail_address(
        db: &DBPool,
        mail_address: &str,
    ) -> Result<Vec<String>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT users
            FROM auths
            WHERE mail_address = $1
            "#,
            mail_address
        )
        .fetch_optional(db)
        .await?;

        let Some(record) = row else {
            return Err(sqlx::Error::RowNotFound);
        };

        Ok(record.users)
    }

    pub async fn update_existing(
        db: &DBPool,
        users: Vec<String>,
        refresh_token: String,
        mail_address: String,
        scopes: Vec<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE auths
            SET users = $1, google_refresh_token = $2, scopes = $3
            WHERE mail_address = $4
            "#,
            &users,
            refresh_token,
            &scopes,
            mail_address
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
