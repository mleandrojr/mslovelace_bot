use sqlx::MySqlPool;

use crate::integrations::telegram::Context;
use crate::integrations::telegram::contexts::{Chat, User};
use crate::repositories::rel_users_chats;

pub async fn run(pool: MySqlPool, ctx: &Context) {
    let (Some(user), Some(chat)) = (&ctx.user, &ctx.chat) else { return };

    let db_user = match ensure_user(&pool, user).await {
        Some(u) => u,
        None => return,
    };
    let db_chat = match ensure_chat(&pool, chat).await {
        Some(c) => c,
        None => return,
    };

    ensure_rel(&pool, db_user.id, db_chat.id).await;
}

async fn ensure_user(pool: &MySqlPool, user: &User) -> Option<crate::models::user::User> {
    match crate::repositories::user::find_by_user_id(pool, user.data.id).await {
        Ok(Some(u)) => { let _ = crate::repositories::user::update(pool, &user.data).await; Some(u) }
        Ok(None) => crate::repositories::user::create(pool, &user.data).await.ok(),
        Err(e) => { eprintln!("DB error (user): {}", e); None }
    }
}

async fn ensure_chat(pool: &MySqlPool, chat: &Chat) -> Option<crate::models::chat::Chat> {
    match crate::repositories::chat::find_by_chat_id(pool, chat.data.id).await {
        Ok(Some(c)) => { let _ = crate::repositories::chat::update(pool, &chat.data).await; Some(c) }
        Ok(None) => crate::repositories::chat::create(pool, &chat.data).await.ok(),
        Err(e) => { eprintln!("DB error (chat): {}", e); None }
    }
}

async fn ensure_rel(pool: &MySqlPool, user_id: u32, chat_id: u32) {
    match rel_users_chats::find(pool, user_id, chat_id).await {
        Ok(Some(_)) => { let _ = rel_users_chats::update_last_seen(pool, user_id, chat_id).await; }
        Ok(None) => { let _ = rel_users_chats::create(pool, user_id, chat_id).await; }
        Err(e) => eprintln!("DB error (rel): {}", e),
    }
}
