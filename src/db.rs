use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;
use dotenvy::dotenv;

pub async fn init_pool() -> MySqlPool {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    
  MySqlPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to connect to MySQL.")

}
