use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn start_connection() -> Result<(), sqlx::Error> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    let result = DB_POOL.set(pool);

    match result {
        Err(_) => panic!("Unable connect to database"),
        Ok(_) => {
            println!("Database Connected");
            Ok(())
        }
    }
}
