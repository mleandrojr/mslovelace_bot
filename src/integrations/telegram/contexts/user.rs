use crate::integrations::telegram::TelegramApi;
use crate::integrations::telegram::types::UserType;
use crate::integrations::telegram::contexts::chat::Chat;

pub struct User {
    api: TelegramApi,
    chat: Chat,
    pub data: UserType
}

impl User {

    pub(crate) fn new(api: TelegramApi, chat: Chat, data: UserType) -> Self {
        Self { api, chat, data }
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
