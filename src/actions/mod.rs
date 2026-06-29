mod adashield;
mod ask_to_ask;
mod blocked_terms;
mod save_user_and_chat;
mod save_message;
mod ping;
mod report;

use sqlx::MySqlPool;
use crate::integrations::telegram::Context;

pub async fn run(pool: MySqlPool, ctx: &Context) {
    save_user_and_chat::run(pool.clone(), ctx).await;
    save_message::run(pool.clone(), ctx).await;
    adashield::run(pool.clone(), ctx).await;
    blocked_terms::run(pool.clone(), ctx).await;
    report::run(pool.clone(), ctx).await;
    ask_to_ask::run(pool.clone(), ctx).await;
    ping::run(pool.clone(), ctx).await;
}
