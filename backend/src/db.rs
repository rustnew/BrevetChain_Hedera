use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;

pub async fn create_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.unwrap()
}