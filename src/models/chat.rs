use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Chat {
    pub id: u32,
    pub federation_id: Option<u32>,
    pub chat_id: i64,
    pub title: String,
    #[sqlx(rename = "type")]
    pub kind: String,
    pub language: String,
    pub joined: bool,
}