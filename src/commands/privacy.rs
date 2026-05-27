use sqlx::MySqlPool;
use crate::integrations::telegram::Context;
use crate::integrations::telegram::params::message::MessageParams;

pub async fn handle(pool: MySqlPool, ctx: &Context) {
    let Some(chat) = &ctx.chat else { return };

    let lang = ctx.t_db(&pool, "privacyPolicy").await;

    let params = MessageParams {
        parse_mode: Some("html".to_string()),
        ..Default::default()
    };

    let _ = chat.send_message(&lang, Option::from(params)).await;
}
