use crate::integrations::telegram::Context;

pub async fn handle(ctx: &Context) {
    let Some(chat) = &ctx.chat else { return };
    chat.send_message(&ctx.t("startMessage"), None).await;
}
