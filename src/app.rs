use std::sync::Arc;

use crate::i18n::I18n;
use crate::utils::log::Log;
use crate::integrations::telegram::TelegramApi;

pub async fn run() {
    dotenv::dotenv().ok();
    Log::init();
    I18n::init();

    let webhook = std::env::var("TELEGRAM_WEBHOOK_ENABLED").unwrap_or_default() == "true";
    let debug = std::env::var("DEBUG").unwrap_or_default() == "true";

    let pool = crate::databases::mysql::connect().await;
    let token = Arc::new(std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set"));
    let api = TelegramApi::new(reqwest::Client::new(), token);

    let run_server = webhook || debug;
    let run_polling = !webhook || debug;

    if run_server && run_polling {
        tokio::join!(
            crate::server::run(pool.clone(), api.clone()),
            crate::polling::run(pool, api)
        );

    } else if run_server {
        crate::server::run(pool, api).await;

    } else if run_polling {
        crate::polling::run(pool, api).await;
    }
}
