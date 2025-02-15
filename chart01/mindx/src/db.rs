use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(100) // Adjust as needed
        .min_connections(1)
        .connect(&database_url)
        .await
}