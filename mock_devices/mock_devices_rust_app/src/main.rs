use sqlx::postgres::PgPoolOptions;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct Rate {
    rate_code: i32,
    description: String,
}

#[async_std::main]
// or #[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost").await?;

    // Make a simple query to return the given parameter
    let row: Vec<Rate> = sqlx::query_as("SELECT * from rates")
        .fetch_all(&pool).await?;

    // assert_eq!(row.0, 150);
    // println!("{:?}", row);

    for rate in &row {
        println!("{:?}\n", rate)
    }

    Ok(())
}