use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder, Result};
use sqlx::{PgPool, Row};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
}

async fn get_user(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        println!("Error fetching user: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(user))
}

async fn create_user(pool: web::Data<PgPool>, user: web::Json<User>) -> Result<impl Responder, Error> {
    let user = user.into_inner();
    let created_user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email",
        user.username,
        user.email
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        println!("Error creating user: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(created_user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
