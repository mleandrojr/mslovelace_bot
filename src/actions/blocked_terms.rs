use crate::integrations::telegram::contexts::Context;
use crate::models::blocked_term::BlockedTermAction;
use crate::repositories::{blocked_terms, chats};
use crate::utils::log::Log;
use sqlx::MySqlPool;

pub async fn run(pool: MySqlPool, ctx: &Context) {
    let Some(ctx_chat) = &ctx.chat else { return };
    let Some(ctx_user) = &ctx.user else { return };

    let false = ctx_user.is_admin().await else { return };

    let Some(message) = &ctx.message else { return };
    let Some(text) = &message.data.text else { return };

    let Ok(Some(db_chat)) = chats::find_by_chat_id(&pool, ctx_chat.data.id).await else { return };

    let terms = match blocked_terms::get_all_from_chat_id(&pool, db_chat.id).await {
        Ok(t) if t.is_empty() => return,
        Ok(t) => t,
        Err(e) => { Log::save(&e.to_string(), true); return; }
    };

    let message_words: Vec<&str> = text.split_whitespace().collect();

    for term in &terms {
        let matched = message_words.iter().any(|word| word.eq_ignore_ascii_case(&term.term));
        if !matched { continue; }

        let _ = message.delete().await;

        match term.action {
            BlockedTermAction::Ban => { ctx_user.ban(None).await; break }
            BlockedTermAction::Mute => { ctx_user.mute(None).await; }
            BlockedTermAction::Warn => { ctx_user.warn(None).await; }
            _ => {}
        }
    }
}
