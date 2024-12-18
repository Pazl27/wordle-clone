use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::env;
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub word: String,
    pub attempts: i32,
    pub score: i32,
    pub name: String,
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
        "INSERT INTO users (id, word, attempts, score, name) VALUES ($1, $2, $3, $4, $5) RETURNING id, word, attempts, score, name",
        new_id,
        word,
        0,
        0,
        "unknown".to_string()
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user(pool: &sqlx::PgPool, id: Uuid) -> sqlx::Result<User> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, word, attempts, score, name FROM users WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn update_user(pool: &sqlx::PgPool, user: &User) -> sqlx::Result<User> {
    let updated_user = sqlx::query_as!(
        User,
        "UPDATE users SET word = $1, attempts = $2, score = $3, name = $4 WHERE id = $5 RETURNING id, word, attempts, score, name",
        user.word,
        user.attempts,
        user.score,
        user.name,
        user.id
    )
    .fetch_one(pool)
    .await?;

    Ok(updated_user)
}

pub async fn get_users(pool: &sqlx::PgPool) -> sqlx::Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT id, word, attempts, score, name FROM users")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;
    use testcontainers::core::ContainerPort;
    use testcontainers::ImageExt;
    use testcontainers::{runners::AsyncRunner, GenericImage};

    #[tokio::test]
    async fn test_create_user() {
        let container_port = ContainerPort::from(5432);
        let container = GenericImage::new("postgres", "latest")
            .with_exposed_port(container_port)
            .with_env_var("POSTGRES_DB", "postgres")
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_HOST_AUTH_METHOD", "trust")
            .start()
            .await
            .unwrap();

        let host = container.get_host().await.unwrap();
        let host_port = container.get_host_port_ipv4(5432).await.unwrap();
        let connection_url =
            format!("postgresql://postgres@{host}:{host_port}/postgres?sslmode=disable");

        tokio::time::sleep(Duration::from_secs(5)).await;

        let connection = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_url)
            .await;

        let pool = connection.expect("Failed to create pool.");

        sqlx::query("CREATE TABLE IF NOT EXISTS users (id UUID PRIMARY KEY, word TEXT, attempts INT, score INT, name TEXT)")
            .execute(&pool)
            .await
            .unwrap();

        let user = create_user(&pool, "test").await.unwrap();

        assert_eq!(user.word, "test");
    }

    #[tokio::test]
    async fn test_get_user() {
        let container_port = ContainerPort::from(5432);
        let container = GenericImage::new("postgres", "latest")
            .with_exposed_port(container_port)
            .with_env_var("POSTGRES_DB", "postgres")
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_HOST_AUTH_METHOD", "trust")
            .start()
            .await
            .unwrap();

        let host = container.get_host().await.unwrap();
        let host_port = container.get_host_port_ipv4(5432).await.unwrap();
        let connection_url =
            format!("postgresql://postgres@{host}:{host_port}/postgres?sslmode=disable");

        tokio::time::sleep(Duration::from_secs(5)).await;

        let connection = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_url)
            .await;

        let pool = connection.expect("Failed to create pool.");

        sqlx::query("CREATE TABLE IF NOT EXISTS users (id UUID PRIMARY KEY, word TEXT, attempts INT, score INT, name TEXT)")
            .execute(&pool)
            .await
            .unwrap();

        let user = create_user(&pool, "test").await.unwrap();
        let user_id = user.id;

        let user = get_user(&pool, user_id).await.unwrap();

        assert_eq!(user.word, "test");
    }

    #[tokio::test]
    async fn test_update_user() {
        let container_port = ContainerPort::from(5432);
        let container = GenericImage::new("postgres", "latest")
            .with_exposed_port(container_port)
            .with_env_var("POSTGRES_DB", "postgres")
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_HOST_AUTH_METHOD", "trust")
            .start()
            .await
            .unwrap();

        let host = container.get_host().await.unwrap();
        let host_port = container.get_host_port_ipv4(5432).await.unwrap();
        let connection_url =
            format!("postgresql://postgres@{host}:{host_port}/postgres?sslmode=disable");

        tokio::time::sleep(Duration::from_secs(5)).await;

        let connection = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_url)
            .await;

        let pool = connection.expect("Failed to create pool.");

        sqlx::query("CREATE TABLE IF NOT EXISTS users (id UUID PRIMARY KEY, word TEXT, attempts INT, score INT, name TEXT)")
            .execute(&pool)
            .await
            .unwrap();

        let user = create_user(&pool, "test").await.unwrap();
        let user_id = user.id;

        let mut user = get_user(&pool, user_id).await.unwrap();
        user.word = "updated".to_string();

        let user = update_user(&pool, &user).await.unwrap();

        assert_eq!(user.word, "updated");
    }

    #[tokio::test]
    async fn test_get_users() {
        let container_port = ContainerPort::from(5432);
        let container = GenericImage::new("postgres", "latest")
            .with_exposed_port(container_port)
            .with_env_var("POSTGRES_DB", "postgres")
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_HOST_AUTH_METHOD", "trust")
            .start()
            .await
            .unwrap();

        let host = container.get_host().await.unwrap();
        let host_port = container.get_host_port_ipv4(5432).await.unwrap();
        let connection_url =
            format!("postgresql://postgres@{host}:{host_port}/postgres?sslmode=disable");

        tokio::time::sleep(Duration::from_secs(5)).await;

        let connection = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_url)
            .await;

        let pool = connection.expect("Failed to create pool.");

        sqlx::query("CREATE TABLE IF NOT EXISTS users (id UUID PRIMARY KEY, word TEXT, attempts INT, score INT, name TEXT)")
            .execute(&pool)
            .await
            .unwrap();

        let _ = create_user(&pool, "test").await.unwrap();
        let _ = create_user(&pool, "test2").await.unwrap();

        let users = get_users(&pool).await.unwrap();

        assert_eq!(users.len(), 2);
    }
}
