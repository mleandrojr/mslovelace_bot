use sqlx::MySqlPool;

pub async fn connect() -> MySqlPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPool::connect(&url).await.expect("Failed to connect to MySQL")
}
