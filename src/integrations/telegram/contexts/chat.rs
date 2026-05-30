use crate::integrations::telegram::params::message::MessageParams;
use crate::integrations::telegram::TelegramApi;
use crate::integrations::telegram::types::{ApiResponse, ChatAdministratorType, ChatType, MessageType};
use crate::utils::log::Log;

#[derive(Clone)]
pub struct Chat {
    api: TelegramApi,
    pub data: ChatType,
}

impl Chat {
    pub(crate) fn new(api: TelegramApi, data: ChatType) -> Self {
        Self { api, data }
    }

    pub async fn get_admins(&self) -> Vec<ChatAdministratorType> {
        let response = match self.api.get_chat_administrators(self.data.id).await {
            Ok(r) => r,
            Err(e) => {
                Log::error(&format!("get_admins failed: {e}"));
                return vec![];
            }
        };

        response.result.unwrap_or_default()
    }

    pub async fn send_message(&self, text: &str, params: Option<MessageParams>) -> ApiResponse<MessageType> {
        self.api.send_message(self.data.id, text, params.as_ref()).await.unwrap_or_else(|e| {
            Log::error(&format!("send_message failed: {e}"));
            ApiResponse { ok: false, result: None, error_code: None, description: None }
        })
    }
}
