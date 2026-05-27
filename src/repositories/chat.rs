use sqlx::MySqlPool;

use crate::models::chat::Chat;
use crate::integrations::telegram::types::{ChatKind, ChatType as TelegramChat};

pub async fn find_by_chat_id(pool: &MySqlPool, chat_id: i64) -> Result<Option<Chat>, sqlx::Error> {
    sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE chat_id = ?")
        .bind(chat_id)
        .fetch_optional(pool)
        .await
}

pub async fn create(pool: &MySqlPool, from: &TelegramChat) -> Result<Chat, sqlx::Error> {
    let title = chat_title(from);
    sqlx::query(
        "INSERT INTO chats (chat_id, title, type) VALUES (?, ?, ?)"
    )
    .bind(from.id)
    .bind(&title)
    .bind(chat_kind(from))
    .execute(pool)
    .await?;

    find_by_chat_id(pool, from.id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn update(pool: &MySqlPool, from: &TelegramChat) -> Result<(), sqlx::Error> {
    let title = chat_title(from);
    sqlx::query(
        "UPDATE chats SET title = ?, type = ? WHERE chat_id = ?"
    )
    .bind(&title)
    .bind(chat_kind(from))
    .bind(from.id)
    .execute(pool)
    .await?;

    Ok(())
}

fn chat_title(from: &TelegramChat) -> String {
    from.title
        .clone()
        .or_else(|| from.first_name.clone())
        .unwrap_or_default()
}

fn chat_kind(from: &TelegramChat) -> &'static str {
    match &from.kind {
        Some(ChatKind::Private) => "private",
        Some(ChatKind::Group) => "group",
        Some(ChatKind::Supergroup) => "supergroup",
        Some(ChatKind::Channel) => "channel",
        None => "",
    }
}