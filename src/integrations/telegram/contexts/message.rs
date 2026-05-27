use crate::integrations::telegram::contexts::UpdateType;
use crate::integrations::telegram::TelegramApi;
use crate::integrations::telegram::types::MessageType;

pub struct Message {
    api: TelegramApi,
    pub data: MessageType,
    pub kind: UpdateType
}

impl Message {
    pub(crate) fn new(api: TelegramApi, data: MessageType, kind: UpdateType) -> Self {
        Self { api, data, kind }
    }

    pub async fn reply(&self, text: &str) -> Result<(), reqwest::Error> {
        self.api.reply_to(self.data.chat.id, self.data.message_id as u64, text, None).await?;
        Ok(())
    }

    pub async fn delete(&self) -> Result<(), reqwest::Error> {
        self.api.delete_message(self.data.chat.id, self.data.message_id as u64).await?;
        Ok(())
    }
}
