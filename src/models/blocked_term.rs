use sqlx::FromRow;

#[derive(Debug, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum BlockedTermAction {
    Delete,
    Mute,
    Warn,
    Ban
}

#[derive(Debug, FromRow)]
pub struct BlockedTerm {
    pub id: u32,
    pub chat_id: u32,
    pub term: String,
    pub(crate) action: BlockedTermAction
}
