use sqlx::MySqlPool;
use crate::integrations::telegram::Context;
use crate::integrations::telegram::contexts::Chat;
use crate::integrations::telegram::contexts::Message;
use crate::integrations::telegram::types::{ChatAdministratorType, MessageType};
use crate::utils::log::Log;
use crate::integrations::telegram::params::message::MessageParams;

pub async fn run(pool: MySqlPool, ctx: &Context) {

    let Some(chat) = &ctx.chat else { return };
    let Some(message) = &ctx.message else { return };
    let Some(admin_mention) = message.mentions.iter().find(|mention| mention.username == "@admin") else {
        return
    };

    report(&pool, &ctx, &chat, &message).await;
}

pub async fn report(pool: &MySqlPool, ctx: &Context, chat: &Chat, message: &Message) {

    let admins: Vec<ChatAdministratorType> = chat.get_admins().await;
    let text_reported = admins.iter().fold(
        ctx.t_db(pool, "reportMessage").await,
        |mut acc, admin| {
            acc.push_str(&format!("[​](tg://user?id={})", admin.user.id));
            acc
        }
    );

    let target = message.reply_to_message.as_deref().unwrap_or(message);
    send_message(target, text_reported.as_str()).await;
}

async fn send_message(message: &Message, text: &str) {
    let params = MessageParams {
        parse_mode: Some("MarkdownV2".to_string()),
        disable_notification: Option::from(false),
        ..Default::default()
    };

    match message.reply(text, Some(params)).await {
        Ok(_) => {}
        Err(error) => Log::error(&error.to_string()),
    }
}
