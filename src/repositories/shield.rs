use sqlx::MySqlPool;
use crate::models::shield::Shield;

pub async fn get_by_user_id(pool: &MySqlPool, user_id: i64) -> Result<Option<Shield>, sqlx::Error> {
    sqlx::query_as::<_, Shield>("SELECT * FROM shield WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

pub async fn get_by_username(pool: &MySqlPool, username: &str) -> Result<Option<Shield>, sqlx::Error> {
    sqlx::query_as::<_, Shield>("SELECT * FROM shield WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await
}
