mod chat;
mod message;
mod user;

pub use chat::Chat;
pub use message::Message;
pub use user::User;

use sqlx::MySqlPool;

use crate::i18n::I18n;
use crate::repositories::chat as chat_repository;
use crate::integrations::telegram::TelegramApi;
use crate::integrations::telegram::types::{
    CallbackQueryType, ChatJoinRequestType, ChatMemberType, ChatMemberUpdatedType, MessageType, UpdateType as TelegramUpdate,
};

#[derive(Clone, Copy)]
pub enum UpdateType {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    CallbackQuery,
    MyChatMember,
    ChatMember,
    ChatJoinRequest,
    Unknown,
}

impl UpdateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UpdateType::Message => "message",
            UpdateType::EditedMessage => "edited_message",
            UpdateType::ChannelPost => "channel_post",
            UpdateType::EditedChannelPost => "edited_channel_post",
            UpdateType::CallbackQuery => "callback_query",
            UpdateType::MyChatMember => "my_chat_member",
            UpdateType::ChatMember => "chat_member",
            UpdateType::ChatJoinRequest => "chat_join_request",
            UpdateType::Unknown => "unknown",
        }
    }
}

pub struct Context {
    pub user: Option<User>,
    pub new_chat_member: Option<ChatMemberType>,
    pub left_chat_member: Option<ChatMemberType>,
    pub chat: Option<Chat>,
    pub message: Option<Message>,
    pub kind: UpdateType,
}

impl Context {
    pub async fn from_update(api: &TelegramApi, update: TelegramUpdate) -> Self {
        if let Some(message) = update.message {
            return Self::from_message(api, message, UpdateType::Message).await;
        }

        if let Some(message) = update.edited_message {
            return Self::from_message(api, message, UpdateType::EditedMessage).await;
        }

        if let Some(message) = update.channel_post {
            return Self::from_message(api, message, UpdateType::ChannelPost).await;
        }

        if let Some(message) = update.edited_channel_post {
            return Self::from_message(api, message, UpdateType::EditedChannelPost).await;
        }

        if let Some(cq) = update.callback_query {
            return Self::from_callback_query(api, cq).await;
        }

        if let Some(cm) = update.my_chat_member {
            return Self::from_chat_member(api, cm, UpdateType::MyChatMember).await;
        }

        if let Some(cm) = update.chat_member {
            return Self::from_chat_member(api, cm, UpdateType::ChatMember).await;
        }

        if let Some(jr) = update.chat_join_request {
            return Self::from_join_request(api, jr).await;
        }

        Self::empty(UpdateType::Unknown)
    }

    async fn from_message(api: &TelegramApi, message: MessageType, kind: UpdateType) -> Self {
        let chat = Chat::new(api.clone(), message.chat.clone());
        let user = message.from.clone().map(|from| User::new(api.clone(), chat.clone(), from));
        let message = Message::new(api.clone(), message, kind);

        Self {
            chat: Some(chat),
            user,
            new_chat_member: None,
            left_chat_member: None,
            message: Some(message),
            kind,
        }
    }

    async fn from_callback_query(api: &TelegramApi, cq: CallbackQueryType) -> Self {
        let chat = cq.message.as_ref().map(|m| Chat::new(api.clone(), m.chat.clone()));
        let user = chat.as_ref().map(|c| User::new(api.clone(), c.clone(), cq.from.clone()));

        Self {
            user,
            new_chat_member: None,
            left_chat_member: None,
            chat,
            message: None,
            kind: UpdateType::CallbackQuery,
        }
    }

    async fn from_chat_member(api: &TelegramApi, cm: ChatMemberUpdatedType, kind: UpdateType) -> Self {
        let chat = Chat::new(api.clone(), cm.chat.clone());
        let user = User::new(api.clone(), chat.clone(), cm.from.clone());

        Self {
            user: Some(user),
            new_chat_member: Some(cm.new_chat_member),
            left_chat_member: None,
            chat: Some(chat),
            message: None,
            kind,
        }
    }

    async fn from_join_request(api: &TelegramApi, jr: ChatJoinRequestType) -> Self {
        let chat = Chat::new(api.clone(), jr.chat.clone());
        let user = User::new(api.clone(), chat.clone(), jr.from.clone());

        Self {
            user: Some(user),
            new_chat_member: None,
            left_chat_member: None,
            chat: Some(chat),
            message: None,
            kind: UpdateType::ChatJoinRequest,
        }
    }

    pub async fn language(&self, pool: &MySqlPool) -> String {
        let chat_id = match self.chat.as_ref() {
            Some(c) => c.data.id,
            None => return "en".to_string(),
        };

        if let Ok(Some(db_chat)) = chat_repository::find_by_chat_id(pool, chat_id).await {
            return db_chat.language;
        }

        self.user.as_ref()
            .and_then(|u| u.data.language_code.clone())
            .unwrap_or_else(|| "en".to_string())
    }

    pub fn t(&self, key: &str) -> String {
        let lang = self.user.as_ref()
            .and_then(|u| u.data.language_code.as_deref())
            .unwrap_or("en");
        I18n::t(lang, key)
    }

    pub async fn t_db(&self, pool: &MySqlPool, key: &str) -> String {
        I18n::t(&self.language(pool).await, key)
    }

    fn empty(kind: UpdateType) -> Self {
        Self { user: None, new_chat_member: None, left_chat_member: None, chat: None, message: None, kind }
    }
}
