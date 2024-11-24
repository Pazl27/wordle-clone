use sqlx::postgres::PgPoolOptions;
use std::env;
use uuid::Uuid;

#[derive(sqlx::FromRow)] pub struct User {
    pub id: Uuid,
    pub attempts: i32,
    pub word: String,
}

pub async fn establish_connection() -> sqlx::Result<sqlx::PgPool> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn create_user(pool: &sqlx::PgPool, word: &str) -> sqlx::Result<User> {
    let new_id = Uuid::new_v4();
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, word, attempts) VALUES ($1, $2, $3) RETURNING id, word, attempts",
        new_id,
        word,
        0
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user(pool: &sqlx::PgPool, id: Uuid) -> sqlx::Result<User> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, word, attempts FROM users WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
