use crate::integrations::telegram::TelegramApi;
use crate::integrations::telegram::types::{ChatKind, UserType};
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

    pub async fn is_admin(&self) -> bool {
        if matches!(self.chat.data.kind, Some(ChatKind::Private)) {
            return true;
        }

        let admins = self.chat.get_admins().await;
        for admin in admins {
            if admin.user.id == self.data.id {
                return true;
            }
        }

        false
    }

    pub async fn mute(&self, until_date: Option<i64>) -> bool {

        if matches!(self.chat.data.kind, Some(ChatKind::Private)) {
            return false;
        }

        use crate::integrations::telegram::types::RestrictType;
        let params = RestrictType {
            permissions: Some(serde_json::json!({
                "can_send_messages": false,
                "can_send_audios": false,
                "can_send_documents": false,
                "can_send_photos": false,
                "can_send_videos": false,
                "can_send_video_notes": false,
                "can_send_voice_notes": false,
                "can_send_polls": false,
                "can_send_other_messages": false
            })),
            until_date,
        };

        self.api.restrict_chat_member(self.data.id, self.chat.data.id, Some(&params)).await.is_ok()
    }

    pub async fn warn(&self, reason: Option<&str>) {

    }

    pub async fn kick(&self, reason: Option<&str>) {

    }

    pub async fn ban(&self, reason: Option<&str>) -> bool {
        self.api.ban_chat_member(self.data.id, self.chat.data.id, None).await.is_ok()
    }

    pub async fn tban(&self, time: u32, reason: Option<&str>) {

    }
}
