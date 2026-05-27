use sqlx::MySqlPool;

use crate::i18n::I18n;
use crate::integrations::telegram::contexts::Context;

pub async fn run(pool: MySqlPool, ctx: &Context) {
    let bot_username = std::env::var("TELEGRAM_USERNAME").unwrap_or_default();

    if !has_mention(ctx, &bot_username) {
        return;
    }

    let Some(chat) = &ctx.chat else { return };
    let text = ctx.t_db(&pool, "pongMessage").await;
    let _ = chat.send_message(&text, None).await;
}

fn has_mention(ctx: &Context, bot_username: &str) -> bool {
    let Some(message) = ctx.message.as_ref() else { return false };

    message.mentions.iter().any(|m| {
        m.username.trim_start_matches('@').eq_ignore_ascii_case(bot_username)
    })
}
