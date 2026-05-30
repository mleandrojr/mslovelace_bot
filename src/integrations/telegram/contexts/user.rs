use crate::integrations::telegram::TelegramApi;
use crate::integrations::telegram::types::{ChatKind, UserType};
use crate::integrations::telegram::contexts::chat::Chat;
use crate::utils::log::Log;

pub struct User {
    api: TelegramApi,
    chat: Chat,
    pub data: UserType
}

impl User {

    pub(crate) fn new(api: TelegramApi, chat: Chat, data: UserType) -> Self {
        Self { api, chat, data }
    }

    pub async fn is_admin(&self) -> bool {
        if matches!(self.chat.data.kind, Some(ChatKind::Private)) {
            Log::debug(&format!("is_admin: user_id={} private chat", self.data.id));
            return true;
        }

        let admins = self.chat.get_admins().await;
        for admin in admins {
            if admin.user.id == self.data.id {
                Log::debug(&format!("is_admin: user_id={} found in admin list", self.data.id));
                return true;
            }
        }

        Log::debug(&format!("is_admin: user_id={} returning false", self.data.id));
        false
    }

    pub fn kick(&self, reason: Option<String>) {

    }

    pub async fn ban(&self, reason: Option<String>) {
        let Ok(_) = self.api.ban_chat_member(self.data.id, self.chat.data.id, None).await else {
            return;
        };
    }

    pub fn tban(&self, time: u32, reason: Option<String>) {

    }
}
