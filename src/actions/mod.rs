mod adashield;
mod save_user_and_chat;
mod save_message;

use sqlx::MySqlPool;
use crate::integrations::telegram::Context;

pub async fn run(pool: MySqlPool, ctx: &Context) {
    save_user_and_chat::run(pool.clone(), ctx).await;
    save_message::run(pool.clone(), ctx).await;
    adashield::run(pool.clone(), ctx).await;
}
