use reqwest::Client;
use sqlx::MySqlPool;
use crate::integrations::combot::CombotApi;
use crate::integrations::telegram::Context;
use crate::repositories::shield;

pub async fn run(pool: MySqlPool, ctx: &Context) {
    let Some(user_data) = ctx.left_chat_member.as_ref().map(|m| &m.user)
        .or_else(|| ctx.new_chat_member.as_ref().map(|m| &m.user))
        .or_else(|| ctx.user.as_ref().map(|u| &u.data)) else { return };

    let Some(chat) = ctx.chat.as_ref() else { return };
    let Some(user) = ctx.user.as_ref() else { return };

    let ban_lang = if shield::get_by_user_id(&pool, user_data.id).await.ok().flatten().is_some() {
        if user.ban(None).await { "adaShieldMessage" } else { "adaShieldMessage2" }

    } else {
        let Ok(response) = CombotApi::new(Client::new()).check(user_data.id).await else { return };
        if !response.ok { return; }

        if user.ban(None).await { "casMessage" } else { "casMessage2" }
    };

    let text = ctx.t_db(&pool, ban_lang).await;
    chat.send_message(&text, None).await;
}
