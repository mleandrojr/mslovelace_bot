use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct RelUserChat {
    pub id: u32,
    pub user_id: u32,
    pub chat_id: u32,
    pub joined: bool,
    pub captcha: Option<String>,
    pub checked: bool,
    pub date: u32,
    pub last_seen: u32,
    pub ttl: Option<u32>,
}