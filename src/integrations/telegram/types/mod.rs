#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatKind {
    Private,
    Group,
    Supergroup,
    Channel,
}

#[derive(Default, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub error_code: Option<u32>,
    pub description: Option<String>,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct AnimationType {
    file_id: String,
    file_unique_id: String,
    width: u32,
    height: u32,
    duration: u32,
    thumbnail: Option<PhotoSizeType>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<u32>
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct AudioType {
    file_id: String,
    file_unique_id: String,
    duration: u32,
    performer: Option<String>,
    title: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<u32>,
    thumbnail: Option<PhotoSizeType>
}

#[derive(Default, serde::Serialize)]
pub struct BanType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

#[derive(Default, serde::Deserialize)]
pub struct CallbackQueryType {
    pub id: String,
    pub from: UserType,
    pub message: Option<MessageType>,
    pub data: Option<String>,
}

#[derive(Default, serde::Deserialize)]
pub struct ChatJoinRequestType {
    pub chat: ChatType,
    pub from: UserType,
    pub date: i64,
}

#[derive(Default, serde::Deserialize)]
pub struct ChatMemberUpdatedType {
    pub chat: ChatType,
    pub from: UserType,
    pub date: i64,
    pub new_chat_member: ChatMemberType,
}

#[derive(Default, serde::Deserialize)]
pub struct ChatMemberType {
    pub user: UserType,
    pub status: String,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatType {
    pub id: i64,
    #[serde(rename = "type")]
    pub kind: Option<ChatKind>,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_forum: Option<bool>,
    pub is_direct_messages: Option<bool>,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct DirectMessagesTopicType {
    topic_id: u64,
    user: UserType
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct DocumentType {
    file_id: String,
    file_unique_id: String,
    thumbnail: Option<PhotoSizeType>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<u64>
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct MessageEntityType {
    pub entity_type: String,
    pub offset: i16,
    pub length: i16,
    pub url: Option<String>,
    pub user: UserType,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>,
    pub unix_time: Option<u64>,
    pub date_time_format: Option<String>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum MessageOriginType {
    MessageOriginUser,
    MessageOriginHiddenUser,
    MessageOriginChat,
    MessageOriginChannel
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct MessageOriginUserType {
    pub origin_type: String,
    pub date: u64,
    pub sender_user: UserType
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct MessageOriginHiddenUserType {
    pub origin_type: String,
    pub date: u64,
    pub sender_user_name: String
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct MessageOriginChatType {
    pub origin_type: String,
    pub date: u64,
    pub sender_chat: ChatType,
    pub author_signature: Option<String>
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct MessageOriginChannelType {
    pub origin_type: String,
    pub date: u64,
    pub chat: ChatType,
    pub message_id: u64,
    author_signature: Option<String>
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct MessageType {
    pub message_id: i32,
    pub message_thread_id: Option<i32>,
    pub direct_messages_topic: Option<DirectMessagesTopicType>,
    pub from: Option<UserType>,
    pub sender_chat: Option<ChatType>,
    pub sender_boost_count: Option<u16>,
    pub sender_business_bot: Option<UserType>,
    pub sender_tag: Option<String>,
    pub date: u64,
    pub guest_query_id: Option<String>,
    pub business_connection_id: Option<String>,
    pub chat: ChatType,
    pub forward_origin: Option<MessageOriginType>,
    pub is_topic_message: Option<bool>,
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<MessageType>>,
    // pub external_reply: Option<ExternalReplyInfo
    pub quote: Option<TextQuoteType>,
    // pub reply_to_story: Option<Story>
    pub reply_to_checklist_task_id: Option<u64>,
    pub reply_to_poll_option_id: Option<String>,
    pub via_bot: Option<UserType>,
    pub guest_bot_caller_user: Option<UserType>,
    pub guest_bot_caller_chat: Option<ChatType>,
    pub edit_date: Option<u64>,
    pub has_protected_content: Option<bool>,
    pub is_from_offline: Option<bool>,
    pub is_paid_post: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub paid_star_count: Option<u16>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntityType>>,
    // pub link_preview_options: Option<LinkPreviewOptions>,
    // pub suggested_post_info: Option<SuggestedPostInfo>,
    pub effect_id: Option<String>,
    pub animation: Option<AnimationType>,
    pub audio: Option<AudioType>,
    pub document: Option<DocumentType>,
    pub photo: Option<Vec<PhotoSizeType>>,
    pub sticker: Option<StickerType>,
    pub video: Option<VideoType>,
    pub video_note: Option<VideoNoteType>,
    pub voice: Option<VoiceType>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntityType>>
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct PhotoSizeType {
    file_id: String,
    file_unique_id: String,
    width: u16,
    height: u16,
    file_size: Option<u64>
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct RestrictType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct StickerType {
    file_id: String,
    file_unique_id: String,
    #[serde(rename = "type")]
    kind: String,
    width: u16,
    height: u16,
    is_animated: bool,
    is_video: bool,
    thumbnail: Option<PhotoSizeType>,
    emoji: Option<String>,
    set_name: Option<String>,
    mask_position: Option<serde_json::Value>,
    custom_emoji_id: Option<String>,
    file_size: Option<u64>
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct TextQuoteType {
    pub text: String,
    pub entities: Vec<MessageEntityType>,
    pub position: u16,
    pub is_manual: Option<bool>
}

#[derive(Default, serde::Deserialize)]
pub struct UpdateType {
    pub update_id: i64,
    pub message: Option<MessageType>,
    pub edited_message: Option<MessageType>,
    pub channel_post: Option<MessageType>,
    pub edited_channel_post: Option<MessageType>,
    pub callback_query: Option<CallbackQueryType>,
    pub my_chat_member: Option<ChatMemberUpdatedType>,
    pub chat_member: Option<ChatMemberUpdatedType>,
    pub chat_join_request: Option<ChatJoinRequestType>,
}

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UserType {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub is_premium: bool,
    pub added_to_attachment_menu: bool,
    pub can_join_groups: bool,
    pub can_read_all_group_messages: bool,
    pub supports_guest_queries: bool,
    pub supports_inline_queries: bool,
    pub can_connect_to_business: bool,
    pub has_main_web_app: bool,
    pub has_topics_enabled: bool,
    pub allows_users_to_create_topics: bool,
    pub can_manage_bots: bool,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct VideoNoteType {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: u16,
    pub duration: u32,
    pub thumbnail: Option<PhotoSizeType>,
    pub file_size: Option<u64>
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct VideoQualityType {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u16,
    pub height: u16,
    pub codec: String,
    pub file_size: Option<u64>
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct VideoType {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u16,
    pub height: u16,
    pub duration: Option<u32>,
    pub thumbnail: Option<PhotoSizeType>,
    pub cover: Option<Vec<PhotoSizeType>>,
    pub start_timestamp: Option<u32>,
    pub qualities: Option<Vec<VideoQualityType>>,
    pub file_name: Option<String>,
    pub file_size: Option<u64>
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct VoiceType {
    file_id: String,
    file_unique_id: String,
    pub duration: Option<u32>,
    mime_type: Option<String>,
    file_size: Option<u32>
}
