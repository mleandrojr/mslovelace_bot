use crate::integrations::telegram::contexts::UpdateType;
use crate::integrations::telegram::TelegramApi;
use crate::integrations::telegram::types::MessageType;

pub struct MentionType {
    pub username: String,
    pub offset: i16,
    pub length: i16,
}

pub struct Message {
    api: TelegramApi,
    pub data: MessageType,
    pub kind: UpdateType,
    pub mentions: Vec<MentionType>,
}

impl Message {
    pub(crate) fn new(api: TelegramApi, data: MessageType, kind: UpdateType) -> Self {
        let mut msg = Self { api, data, kind, mentions: vec![] };
        msg.parse_entities();
        msg
    }

    fn parse_entities(&mut self) {
        let Some(entities) = self.data.entities.clone() else { return };

        self.mentions = entities.iter()
            .filter(|e| e.kind == "mention")
            .filter_map(|e| Self::parse_mention(&self.data, e.offset, e.length))
            .collect();
    }

    fn parse_mention(data: &MessageType, offset: i16, length: i16) -> Option<MentionType> {
        let text = data.text.as_ref()?;
        let chars: Vec<char> = text.chars().collect();
        let start = offset as usize;
        let end = start + length as usize;
        if end > chars.len() { return None; }
        let username: String = chars[start..end].iter().collect();
        Some(MentionType { username, offset, length })
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
