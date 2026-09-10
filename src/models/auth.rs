use sqlx::types::Json;
use sqlx::types::uuid;

use crate::db::DBPool;
use crate::types::platform::Platform;

#[derive(sqlx::FromRow, Debug)]
pub struct Auth {
    pub uuid: uuid::Uuid,
    pub platform_users: Json<Vec<Platform>>,
    pub refresh_token: String,
    pub mail_address: String,
    pub scopes: Vec<String>,
}

impl Auth {
    pub async fn insert(
        db: &DBPool,
        platform_users: Vec<Platform>,
        refresh_token: String,
        mail_address: String,
        scopes: Vec<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO auths (platform_users, refresh_token, mail_address, scopes)
            VALUES ($1, $2, $3, $4)
            "#,
            Json(platform_users) as _,
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
        platform_user: &Platform,
    ) -> Result<Auth, sqlx::Error> {
        let platform_user_json = serde_json::to_value([platform_user]).unwrap();

        let row = sqlx::query_as!(
            Auth,
            r#"
            SELECT 
                uuid, 
                platform_users as "platform_users: Json<Vec<Platform>>", 
                refresh_token, 
                mail_address, 
                scopes
            FROM auths
            WHERE platform_users @> $1
            "#,
            platform_user_json
        )
        .fetch_one(db)
        .await?;

        Ok(row)
    }

    pub async fn get_platform_users_by_mail_address(
        db: &DBPool,
        mail_address: &str,
    ) -> Result<Vec<Platform>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT platform_users as "platform_users: Json<Vec<Platform>>"
            FROM auths
            WHERE mail_address = $1
            "#,
            mail_address
        )
        .fetch_one(db)
        .await?;

        Ok(row.platform_users.0)
    }

    pub async fn update_existing(
        db: &DBPool,
        platform_users: Vec<Platform>,
        refresh_token: String,
        mail_address: String,
        scopes: Vec<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE auths
            SET platform_users = $1, refresh_token = $2, scopes = $3
            WHERE mail_address = $4
            "#,
            Json(platform_users) as _,
            refresh_token,
            &scopes,
            mail_address
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
