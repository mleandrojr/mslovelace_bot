use sqlx::MySqlPool;

use crate::integrations::telegram::Context;
use crate::integrations::telegram::contexts::Message;
use crate::repositories::{chat, message, user};

pub async fn run(pool: MySqlPool, ctx: &Context) {
    let (Some(ctx_user), Some(ctx_chat), Some(ctx_message)) = (&ctx.user, &ctx.chat, &ctx.message) else { return };

    let Ok(Some(db_user)) = user::find_by_user_id(&pool, ctx_user.data.id).await else { return };
    let Ok(Some(db_chat)) = chat::find_by_chat_id(&pool, ctx_chat.data.id).await else { return };

    let db_message = match ensure_message(&pool, db_user.id, db_chat.id, ctx_message).await {
        Some(m) => m,
        None => return,
    };

    if let Some(reply_to) = &ctx_message.data.reply_to_message {
        add_reply_to_message_id(&pool, &db_message, reply_to.message_id).await;
    }

    if let Some(thread_id) = ctx_message.data.message_thread_id {
        add_thread_id(&pool, &db_message, thread_id).await;
    }
}

async fn ensure_message(
    pool: &MySqlPool,
    user_id: u32,
    chat_id: u32,
    ctx_message: &Message,
) -> Option<crate::models::message::Message> {
    match message::find_by_telegram_id(pool, ctx_message.data.message_id, chat_id).await {
        Ok(Some(m)) => { let _ = message::update(pool, ctx_message).await; Some(m) }
        Ok(None) => message::create(pool, user_id, chat_id, ctx_message).await.ok(),
        Err(e) => { eprintln!("DB error (message): {}", e); None }
    }
}

async fn add_reply_to_message_id(pool: &MySqlPool, db_message: &crate::models::message::Message, reply_to_message_id: i32) {
    match message::find_by_telegram_id(pool, reply_to_message_id, db_message.chat_id).await {
        Ok(Some(m)) => { let _ = message::update_reply_to(pool, db_message, m.id).await; }
        Ok(None) => (),
        Err(e) => eprintln!("DB error (reply_to): {}", e),
    }
}

async fn add_thread_id(pool: &MySqlPool, db_message: &crate::models::message::Message, thread_id: i32) {
    match message::find_by_telegram_id(pool, thread_id, db_message.chat_id).await {
        Ok(Some(m)) => { let _ = message::update_thread_id(pool, db_message, m.id).await; }
        Ok(None) => (),
        Err(e) => eprintln!("DB error (thread_id): {}", e),
    }
}
