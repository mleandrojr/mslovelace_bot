use crate::integrations::telegram::TelegramApi;
use crate::integrations::telegram::types::{ChatType, MessageType};

#[derive(Clone)]
pub struct Chat {
    api: TelegramApi,
    pub data: ChatType,
}

impl Chat {
    pub(crate) fn new(api: TelegramApi, data: ChatType) -> Self {
        Self { api, data }
    }

    pub async fn send_message(&self, text: &str, params: Option<MessageType>) -> Result<(), reqwest::Error> {
        self.api.send_message(self.data.id as i64, text, params.as_ref()).await?;
        Ok(())
    }
}
