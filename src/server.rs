use axum::Router;
use sqlx::MySqlPool;
use tokio::net::TcpListener;

use crate::routes;
use crate::integrations::telegram::TelegramApi;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub api: TelegramApi,
    pub auth: String,
}

pub async fn run(pool: MySqlPool, api: TelegramApi) {
    let auth = std::env::var("AUTH").expect("AUTH must be set");
    let state = AppState { pool, api, auth };
    let app = Router::new().merge(routes::router()).with_state(state);

    let base_port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let mut port = base_port;
    let listener = loop {
        let addr = format!("0.0.0.0:{}", port);
        match TcpListener::bind(&addr).await {
            Ok(l) => {
                println!("Listening on {}", addr);
                break l;
            }
            Err(e) => {
                eprintln!("Port {} unavailable ({}), trying {}...", port, e, port + 1);
                port += 1;
            }
        }
    };

    axum::serve(listener, app).await.expect("Server error");
}
