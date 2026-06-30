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
    let Some(_) = message.mentions.iter().find(|mention| mention.username == "@admin") else {
        return
    };

    report(&pool, &ctx, &chat, &message).await;
}

pub async fn report(pool: &MySqlPool, ctx: &Context, chat: &Chat, message: &Message) {

    let target = message.reply_to_message.as_deref().unwrap_or(message);

    if is_self_message(target) {
        let text = ctx.t_db(pool, "selfReportMessage").await;
        send_message(target, text.as_str()).await;
        return;
    }

    let admins: Vec<ChatAdministratorType> = chat.get_admins().await;
    if is_admin_message(target, &admins) {
        let text = ctx.t_db(pool, "adminReportMessage").await;
        send_message(target, text.as_str()).await;
        return;
    }

    let text_reported = admins.iter().fold(
        ctx.t_db(pool, "reportMessage").await,
        |mut acc, admin| {
            acc.push_str(&format!("[​](tg://user?id={})", admin.user.id));
            acc
        }
    );

    send_message(target, text_reported.as_str()).await;
}

fn is_self_message(message: &Message) -> bool {
    let bot_id: i64 = std::env::var("TELEGRAM_USER_ID")
        .expect("TELEGRAM_USER_ID must be set")
        .parse()
        .expect("Failed to parse TELEGRAM_USER_ID");

    message.data.from.as_ref().is_some_and(|from| from.id == bot_id)
}

fn is_admin_message(message: &Message, admins: &Vec<ChatAdministratorType>) -> bool {
    let Some(from) = &message.data.from else { return false };
    admins.iter().any(|admin| admin.user.id == from.id)
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
