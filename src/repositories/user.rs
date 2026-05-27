use sqlx::MySqlPool;

use crate::integrations::telegram::types::UserType as TelegramUser;
use crate::models::user::User;

pub async fn find_by_user_id(pool: &MySqlPool, user_id: i64) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

pub async fn create(pool: &MySqlPool, from: &TelegramUser) -> Result<User, sqlx::Error> {
    sqlx::query(
        "INSERT INTO users (user_id, username, first_name, last_name, is_bot, is_premium, language_code)
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(from.id)
    .bind(&from.username)
    .bind(&from.first_name)
    .bind(&from.last_name)
    .bind(from.is_bot)
    .bind(from.is_premium)
    .bind(&from.language_code)
    .execute(pool)
    .await?;

    find_by_user_id(pool, from.id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn update(pool: &MySqlPool, from: &TelegramUser) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users SET username = ?, first_name = ?, last_name = ?, is_premium = ?, language_code = ?
         WHERE user_id = ?"
    )
    .bind(&from.username)
    .bind(&from.first_name)
    .bind(&from.last_name)
    .bind(from.is_premium)
    .bind(&from.language_code)
    .bind(from.id)
    .execute(pool)
    .await?;

    Ok(())
}