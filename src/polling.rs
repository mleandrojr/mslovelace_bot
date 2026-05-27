use sqlx::MySqlPool;

use crate::services::telegram;
use crate::integrations::telegram::TelegramApi;
use crate::utils::log::Log;

pub async fn run(pool: MySqlPool, api: TelegramApi) {
    let mut offset: i64 = 0;

    loop {
        Log::info(&format!("Getting updates from offset {}", offset.to_string()));

        match api.get_updates(offset, 30).await {
            Ok(response) => {
                for update in response.result.unwrap_or_default() {
                    offset = update.update_id + 1;
                    telegram::process(&pool, &api, update).await;
                }
            }

            Err(e) => {
                eprintln!("Polling error: {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}
