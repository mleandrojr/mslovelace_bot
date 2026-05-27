use sqlx::MySqlPool;
use crate::integrations::telegram::Context;

pub async fn handle(pool: MySqlPool, ctx: &Context) {
    let Some(chat) = &ctx.chat else { return };

    let lang = ctx.t_db(&pool, "startMessage").await;
    let _ = chat.send_message(&lang, None).await;
}
