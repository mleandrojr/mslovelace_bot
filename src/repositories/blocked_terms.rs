use sqlx::{Error, MySqlPool};

use crate::models::blocked_term::BlockedTerm;

pub async fn get_all_from_chat_id(pool: &MySqlPool, chat_id: u32) -> Result<Vec<BlockedTerm>, Error> {
    sqlx::query_as::<_, BlockedTerm>("SELECT * FROM blocked_terms WHERE chat_id = ?")
        .bind(chat_id)
        .fetch_all(pool)
        .await
}
