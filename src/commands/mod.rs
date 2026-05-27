mod start;
mod privacy;

use sqlx::MySqlPool;
use crate::integrations::telegram::Context;

pub async fn dispatch(pool: MySqlPool, ctx: &Context, command: &str) {
    match command {
        "start" => start::handle(pool.clone(), ctx).await,
        "privacy" => privacy::handle(pool.clone(), ctx).await,
        _ => {}
    }
}
