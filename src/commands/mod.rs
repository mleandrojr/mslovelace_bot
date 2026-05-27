mod start;
mod privacy;

use crate::integrations::telegram::Context;

pub async fn dispatch(ctx: &Context, command: &str) {
    match command {
        "start" => start::handle(ctx).await,
        "privacy" => privacy::handle(ctx).await,
        _ => {}
    }
}
