use sqlx::MySqlPool;

use crate::models::rel_user_chat::RelUserChat;

pub async fn find(pool: &MySqlPool, user_id: u32, chat_id: u32) -> Result<Option<RelUserChat>, sqlx::Error> {
    sqlx::query_as::<_, RelUserChat>(
        "SELECT * FROM rel_users_chats WHERE user_id = ? AND chat_id = ?"
    )
    .bind(user_id)
    .bind(chat_id)
    .fetch_optional(pool)
    .await
}

pub async fn create(pool: &MySqlPool, user_id: u32, chat_id: u32) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().timestamp() as u32;
    sqlx::query(
        "INSERT INTO rel_users_chats (user_id, chat_id, date, last_seen) VALUES (?, ?, ?, ?)"
    )
    .bind(user_id)
    .bind(chat_id)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_last_seen(pool: &MySqlPool, user_id: u32, chat_id: u32) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().timestamp() as u32;
    sqlx::query(
        "UPDATE rel_users_chats SET last_seen = ? WHERE user_id = ? AND chat_id = ?"
    )
    .bind(now)
    .bind(user_id)
    .bind(chat_id)
    .execute(pool)
    .await?;

    Ok(())
}