use sqlx::{Error, MySqlPool};
use sqlx::mysql::MySqlQueryResult;
use crate::models::message::Message;

fn to_json<T: serde::Serialize>(v: &T) -> String {
    serde_json::to_string(v).unwrap_or_default()
}

pub async fn find_by_telegram_id(pool: &MySqlPool, telegram_id: i32, chat_id: u32) -> Result<Option<Message>, Error> {
    sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE message_id = ? AND chat_id = ?")
        .bind(telegram_id)
        .bind(chat_id)
        .fetch_optional(pool)
        .await
}

pub async fn create(pool: &MySqlPool, user_id: u32, chat_id: u32, message: &crate::integrations::telegram::contexts::Message) -> Result<Message, Error> {
    let result = sqlx::query("INSERT INTO messages (
        user_id,
        chat_id,
        message_id,
        type,
        content,
        entities,
        animation,
        audio,
        document,
        photo,
        sticker,
        video,
        video_note,
        voice,
        caption,
        caption_entities,
        date,
        ttl,
        status
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);")
        .bind(user_id)
        .bind(chat_id)
        .bind(message.data.message_id)
        .bind(message.kind.as_str())
        .bind(&message.data.text)
        .bind(serde_json::to_string(&message.data.entities).unwrap_or_default())
        .bind(serde_json::to_string(&message.data.animation).unwrap_or_default())
        .bind(serde_json::to_string(&message.data.audio).unwrap_or_default())
        .bind(serde_json::to_string(&message.data.document).unwrap_or_default())
        .bind(serde_json::to_string(&message.data.photo).unwrap_or_default())
        .bind(serde_json::to_string(&message.data.sticker).unwrap_or_default())
        .bind(serde_json::to_string(&message.data.video).unwrap_or_default())
        .bind(serde_json::to_string(&message.data.video_note).unwrap_or_default())
        .bind(serde_json::to_string(&message.data.voice).unwrap_or_default())
        .bind(&message.data.caption)
        .bind(serde_json::to_string(&message.data.caption_entities).unwrap_or_default())
        .bind(message.data.date)
        .bind(None::<u32>)
        .bind(1)
        .execute(pool)
        .await?;

    let id = result.last_insert_id() as u32;

    let select = sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await;

    if let Err(ref e) = select {
        eprintln!("SELECT after INSERT failed (id={}): {:?}", id, e);
    }

    select
}

pub async fn update(pool: &MySqlPool, message: &crate::integrations::telegram::contexts::Message) -> Result<Message, sqlx::Error> {
    let d = &message.data;

    let fields: Vec<(&str, Option<String>)> = vec![
        ("type", Some(message.kind.as_str().to_string())),
        ("content", d.text.clone()),
        ("entities", d.entities.as_ref().map(to_json)),
        ("caption", d.caption.clone()),
        ("caption_entities", d.caption_entities.as_ref().map(to_json)),
        ("animation", d.animation.as_ref().map(to_json)),
        ("audio", d.audio.as_ref().map(to_json)),
        ("document", d.document.as_ref().map(to_json)),
        ("photo", d.photo.as_ref().map(to_json)),
        ("sticker", d.sticker.as_ref().map(to_json)),
        ("video", d.video.as_ref().map(to_json)),
        ("video_note", d.video_note.as_ref().map(to_json)),
        ("voice", d.voice.as_ref().map(to_json)),
    ];

    let mut qb = sqlx::QueryBuilder::new("UPDATE messages SET ");
    let mut sep = qb.separated(", ");
    for (col, val) in fields {
        if let Some(v) = val {
            sep.push(format!("{col} = ")).push_bind(v);
        }
    }

    qb.push(" WHERE message_id = ").push_bind(d.message_id);
    qb.build().execute(pool).await?;

    sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE message_id = ?")
        .bind(d.message_id)
        .fetch_one(pool)
        .await
}

pub async fn update_thread_id(pool: &MySqlPool, message: &Message, thread_id: u32) -> Result<MySqlQueryResult, Error> {
    sqlx::query("UPDATE messages SET thread_id = ? WHERE id = ?")
        .bind(thread_id)
        .bind(message.id)
        .execute(pool)
        .await
}

pub async fn update_reply_to(pool: &MySqlPool, message: &Message, reply_to: u32) -> Result<MySqlQueryResult, Error> {
    sqlx::query("UPDATE messages SET reply_to = ? WHERE id = ?")
        .bind(reply_to)
        .bind(message.id)
        .execute(pool)
        .await
}

pub async fn update_ttl(pool: &MySqlPool, message: &Message, ttl: u32) -> Result<MySqlQueryResult, Error> {
    sqlx::query("UPDATE messages SET ttl = ? WHERE id = ?")
        .bind(ttl)
        .bind(message.id)
        .execute(pool)
        .await
}