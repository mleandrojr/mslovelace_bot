use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Shield {
    pub id: u32,
    pub user_id: i64,
    pub username: String,
    pub date: u32,
    pub reason: String
}
