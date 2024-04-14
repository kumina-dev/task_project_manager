use sqlx::{FromRow, PgPool, Pool, Postgres, Result};

#[derive(Debug, FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
}

pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn init_pool() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::builder()
        .max_size(10)
        .build(&database_url)
        .await
        .expect("Failed to create pool")
}

fn main() {
    println!("Hello, world!");
}
