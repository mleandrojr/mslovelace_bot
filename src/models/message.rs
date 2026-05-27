use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Message {
    pub id: u32,
    pub user_id: u32,
    pub chat_id: u32,
    pub thread_id: Option<u32>,
    pub message_id: u32,
    #[sqlx(rename = "type")]
    pub kind: String,
    pub reply_to: Option<u32>,
    pub content: Option<String>,
    pub callback_query: Option<String>,
    pub entities: Option<String>,
    pub animation: Option<String>,
    pub audio: Option<String>,
    pub document: Option<String>,
    pub photo: Option<String>,
    pub sticker: Option<String>,
    pub video: Option<String>,
    pub video_note: Option<String>,
    pub voice: Option<String>,
    pub caption: Option<String>,
    pub caption_entities: Option<String>,
    pub date: u32,
    pub ttl: Option<u32>,
    pub status: bool,
}