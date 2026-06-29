use sqlx::MySqlPool;

use crate::actions;
use crate::commands;
use crate::integrations::telegram::Context;
use crate::integrations::telegram::types::UpdateType;
use crate::integrations::telegram::TelegramApi;

pub async fn process(pool: &MySqlPool, api: &TelegramApi, update: UpdateType) {
    let ctx = Context::from_update(api, update).await;

    actions::run(pool.clone(), &ctx).await;

    if let Some(cmd) = extract_command(&ctx) {
        commands::dispatch(pool.clone(), &ctx, &cmd).await;
    }
}

fn extract_command(ctx: &Context) -> Option<String> {
    let text = ctx.message.as_ref()?.data.text.as_deref()?;
    if !text.starts_with('/') {
        return None;
    }

    let command = text
        .split_whitespace()
        .next()?
        .trim_start_matches('/')
        .split('@')
        .next()?
        .to_lowercase();
    Some(command)
}
