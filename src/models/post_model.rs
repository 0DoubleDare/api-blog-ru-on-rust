use crate::models::tables::*;

// "extract" - извлекает данные
use axum::extract::{State, Path, Form};
use axum::{
    Json,
    // Статус коды
    http::StatusCode,
};
use sqlx::MySqlPool;
pub async fn get_post_by_id(State(pool): State<MySqlPool>, Path(id): Path<i32>)
    -> Json<Option<Post>>
{
    let result = sqlx::query_as!(
        Post,
        "SELECT * FROM posts WHERE id = ?",
        id
    ).fetch_optional(&pool).await;
    match result {
        Ok(post) => {
            tracing::info!("GET: Success to get post by id: ");
            Json(post)
        }
        Err(error) => {
            tracing::error!("GET: Failed to get post by id: {error}");
            Json(None)
        }
    }
}

pub async fn get_posts(State(pool): State<MySqlPool>) -> Json<Vec<Post>> {
    let result = sqlx::query_as!(
        Post,
        "SELECT * FROM posts"
    ).fetch_all(&pool).await;

    match result {
        Ok(posts) => {
            tracing::info!("GET: Success get `posts`");
            Json(posts)
        }
        Err(error) => {
            tracing::error!("GET: Failed to get `posts` table: {error}");
            Json(vec![])
        }
    }
}

pub async fn add_post(State(pool): State<MySqlPool>, Form(payload): Form<AddPost>)
    -> Result<(StatusCode, Json<Post>), StatusCode>
{
    let result = sqlx::query(
        "INSERT INTO posts(title, description, author_id) VALUES (?, ?, ?)"
    ).bind(&payload.title).bind(&payload.description).bind(&payload.author_id)
        .execute(&pool).await;

    let new_post_id = result.unwrap().last_insert_id();
    let new_post = Post {
        id: new_post_id as i32,
        title: Some(payload.title.unwrap()),
        description: Some(payload.description.unwrap()),
        author_id: payload.author_id
    };
    tracing::info!("POST: Success add post in table");
    Ok((StatusCode::CREATED, Json(new_post)))
}