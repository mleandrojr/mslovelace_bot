use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: u32,
    pub user_id: i64,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_channel: bool,
    pub is_bot: bool,
    pub is_premium: bool,
    pub language_code: Option<String>,
}
