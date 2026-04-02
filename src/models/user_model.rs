use crate::models::tables::*;
use axum::extract::{State, Path, Form};
use axum::{
    Json,
    http::StatusCode
};
use sqlx::MySqlPool;

pub async fn add_user(State(pool): State<MySqlPool>, Form(payload): Form<AddUser>)
    -> Result<(StatusCode, Json<User>), StatusCode>
{
    let result = sqlx::query(
        "INSERT INTO users(name) VALUES (?)"
    ).bind(&payload.name).execute(&pool).await;

    let user = User {
        id: result.unwrap().last_insert_id() as i32,
        name: Some(payload.name)
    };
    tracing::trace!("POST: Success to add user in table");
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_users(State(pdo): State<MySqlPool>) -> Json<Vec<User>> {
    let result = sqlx::query_as!(
        User,
        "SELECT * FROM users"
    ).fetch_all(&pdo).await;

    match result {
        Ok(users) => {
            Json(users)
        }
        Err(_) => {
            Json(vec![])
        }
    }
}

pub async fn get_user_by_id(State(pdo): State<MySqlPool>, Path(id): Path<i32>)
    -> Json<Option<User>>
{
    let result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = ?",
        id
    ).fetch_optional(&pdo).await;

    match result {
        Ok(user) => {
            Json(user)
        }
        Err(_) => {
            Json(None)
        }
    }
}