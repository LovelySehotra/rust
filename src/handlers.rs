use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreateUser, User};

/// CREATE
pub async fn create_user(
    State(db): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Json<User> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, name, email)
         VALUES ($1, $2, $3)
         RETURNING id, name, email",
        Uuid::new_v4(),
        payload.name,
        payload.email
    )
    .fetch_one(&db)
    .await
    .unwrap();

    Json(user)
}

/// READ ALL
pub async fn get_users(
    State(db): State<PgPool>,
) -> Json<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users"
    )
    .fetch_all(&db)
    .await
    .unwrap();

    Json(users)
}

/// READ ONE
pub async fn get_user(
    Path(id): Path<Uuid>,
    State(db): State<PgPool>,
) -> Json<User> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    Json(user)
}

/// UPDATE
pub async fn update_user(
    Path(id): Path<Uuid>,
    State(db): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Json<User> {
    let user = sqlx::query_as!(
        User,
        "UPDATE users
         SET name = $1, email = $2
         WHERE id = $3
         RETURNING id, name, email",
        payload.name,
        payload.email,
        id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    Json(user)
}

/// DELETE
pub async fn delete_user(
    Path(id): Path<Uuid>,
    State(db): State<PgPool>,
) -> Json<&'static str> {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&db)
        .await
        .unwrap();

    Json("User deleted successfully")
}
