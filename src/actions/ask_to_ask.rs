use regex::RegexBuilder;
use sqlx::MySqlPool;
use crate::integrations::telegram::Context;
use crate::integrations::telegram::params::message::MessageParams;

pub async fn run(pool: MySqlPool, ctx: &Context) {
    let Some(chat) = &ctx.chat else { return };
    let Some(user) = &ctx.user else { return };
    let false = user.is_admin().await else { return };

    let Some(message) = &ctx.message else { return; };
    let Some(text) = &message.data.text else { return; };

    let pattern = ctx.t_db(&pool, "askToAskRegex").await;
    let Some(true) = matches_regex(&pattern, text) else { return; };

    let _ = message.delete().await;

    let display_name = user.data.first_name.as_deref()
        .filter(|s| !s.is_empty())
        .or(user.data.username.as_deref())
        .unwrap_or("")
        .to_string();

    let reply = ctx.t_db(&pool, "askToAskLink").await
        .replace("{userid}", &user.data.id.to_string())
        .replace("{username}", &display_name);

    let params = MessageParams {
        parse_mode: Some("html".to_string()),
        ..Default::default()
    };

    let _ = chat.send_message(&reply, Option::from(params)).await;
}

fn matches_regex(js_pattern: &str, text: &str) -> Option<bool> {
    let inner = js_pattern.strip_prefix('/')?;
    let last_slash = inner.rfind('/')?;
    let (pattern, flags) = (&inner[..last_slash], &inner[last_slash + 1..]);
    let re = RegexBuilder::new(pattern)
        .case_insensitive(flags.contains('i'))
        .build()
        .ok()?;

    Some(re.is_match(text))
}
