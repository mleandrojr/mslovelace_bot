pub mod contexts;
pub mod types;

pub(crate) mod params;

pub use contexts::Context;

use std::sync::Arc;
use reqwest::Client;
use types::{ApiResponse, BanType, ChatAdministratorType, MessageType, RestrictType, UpdateType};
use crate::integrations::telegram::params::message::MessageParams;

#[derive(Clone)]
pub struct TelegramApi {
    client: Client,
    token: Arc<String>,
}

impl TelegramApi {
    pub fn new(client: Client, token: Arc<String>) -> Self {
        Self { client, token }
    }

    fn url(&self, method: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.token, method)
    }

    fn merge(base: &mut serde_json::Value, params: &impl serde::Serialize) {
        let Ok(extra) = serde_json::to_value(params) else {
            return;
        };

        let (Some(base), Some(extra)) = (base.as_object_mut(), extra.as_object()) else {
            return;
        };

        base.extend(extra.iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    pub async fn get_updates(&self, offset: i64, timeout: u32) -> Result<ApiResponse<Vec<UpdateType>>, reqwest::Error> {
        let url = format!("{}?offset={}&timeout={}", self.url("getUpdates"), offset, timeout);
        self.client.get(&url).send().await?.json().await
    }

    pub async fn get_chat_administrators(&self, chat_id: i64) -> Result<ApiResponse<Vec<ChatAdministratorType>>, reqwest::Error> {
        let body = serde_json::json!({
            "chat_id": chat_id
        });

        self.client.post(self.url("getChatAdministrators")).json(&body).send().await?.json().await
    }

    pub async fn send_message(&self, chat_id: i64, text: &str, params: Option<&MessageParams>) -> Result<ApiResponse<MessageType>, reqwest::Error> {
        let mut body = serde_json::json!({ "chat_id": chat_id, "text": text });
        if let Some(p) = params { Self::merge(&mut body, p); }
        self.client.post(self.url("sendMessage")).json(&body).send().await?.json().await
    }

    pub async fn reply_to(&self, chat_id: i64, message_id: u64, text: &str, params: Option<&MessageParams>) -> Result<ApiResponse<MessageType>, reqwest::Error> {
        let mut body = serde_json::json!({
            "chat_id": chat_id,
            "text": text,
            "reply_to_message_id": message_id,
        });

        if let Some(p) = params { Self::merge(&mut body, p); }
        self.client.post(self.url("sendMessage")).json(&body).send().await?.json().await
    }

    pub async fn delete_message(&self, chat_id: i64, message_id: u64) -> Result<ApiResponse<bool>, reqwest::Error> {
        self.client
            .post(self.url("deleteMessage"))
            .json(&serde_json::json!({ "chat_id": chat_id, "message_id": message_id }))
            .send()
            .await?.json().await
    }

    pub async fn ban_chat_member(&self, user_id: i64, chat_id: i64, params: Option<&BanType>) -> Result<ApiResponse<bool>, reqwest::Error> {
        let mut body = serde_json::json!({ "chat_id": chat_id, "user_id": user_id });
        if let Some(p) = params { Self::merge(&mut body, p); }
        self.client.post(self.url("banChatMember")).json(&body).send().await?.json().await
    }

    pub async fn restrict_chat_member(&self, user_id: i64, chat_id: i64, params: Option<&RestrictType>) -> Result<ApiResponse<bool>, reqwest::Error> {
        let mut body = serde_json::json!({ "chat_id": chat_id, "user_id": user_id });
        if let Some(p) = params { Self::merge(&mut body, p); }
        self.client.post(self.url("restrictChatMember")).json(&body).send().await?.json().await
    }

    pub async fn approve_chat_join_request(&self, user_id: i64, chat_id: i64) -> Result<ApiResponse<bool>, reqwest::Error> {
        self.client
            .post(self.url("approveChatJoinRequest"))
            .json(&serde_json::json!({ "chat_id": chat_id, "user_id": user_id }))
            .send()
            .await?.json().await
    }

    pub async fn decline_chat_join_request(&self, user_id: i64, chat_id: i64) -> Result<ApiResponse<bool>, reqwest::Error> {
        self.client
            .post(self.url("declineChatJoinRequest"))
            .json(&serde_json::json!({ "chat_id": chat_id, "user_id": user_id }))
            .send()
            .await?.json().await
    }
}
