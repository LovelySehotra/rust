use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    let app = Router::new()
        .route("/users", post(handlers::create_user).get(handlers::get_users))
        .route(
            "/users/:id",
            get(handlers::get_user)
                .put(handlers::update_user)
                .delete(handlers::delete_user),
        )
        .with_state(pool);

    println!("🚀 Server running on http://localhost:3000");

    // ✅ AXUM 0.7 SERVER (THIS IS THE FIX)
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port");

    axum::serve(listener, app).await.unwrap();
}
