mod tables;

use tables::*;
use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{State, Path, Form},
    http::StatusCode
};
use serde_json::json;
use serde::Serialize;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let database_url = "mariadb://admin:12345678@127.0.0.1:3306/rust_api_test";
    let pool =
        match MySqlPoolOptions::new().max_connections(5).connect(database_url).await {
            Ok(result) => {
                tracing::info!("Succes connect to database ({database_url})");
                result
            },
            Err(error) => {
                tracing::error!("Error with connect to database. Url - {database_url}");
                panic!("Message: {error}");
            }
        };

    let api_app = Router::new()
        .route("/", get(async || { "Hello Axum" }))
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user_by_id))
        .route("/users", post(add_user))
        .route("/posts", get(get_posts))
        .route("/posts/:id", get(get_post_by_id))
        .route("/posts", post(add_post))
        .with_state(pool);

    let url = "127.0.0.1:3030";
    let listener =
        tokio::net::TcpListener::bind(url).await.unwrap();

    tracing::info!("Open server in http://{url}");
    axum::serve(listener, api_app).await.unwrap();
}

async fn get_post_by_id(State(pool): State<MySqlPool>, Path(id): Path<i32>) {
    todo!("Реализовать получение поста по ID")
}
async fn add_post(State(pool): State<MySqlPool>, Form(payload): Form<AddPost>)
    -> Result<(StatusCode, Json<Post>), StatusCode> {
    let result = sqlx::query(
        "INSERT INTO posts(title, description, author_id) VALUES (?, ?, ?)"
    ).bind(&payload.title).bind(&payload.description).bind(&payload.author_id)
        .execute(&pool).await
        .map_err(
            |error| {
                tracing::info!("Add post error: {error}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    let new_post_id = result.last_insert_id();

    let post =  Post {
        id: new_post_id as i32,
        title: Some(payload.title.unwrap()),
        description: Some(payload.description.unwrap()),
        author_id: payload.author_id
    };

    Ok((StatusCode::CREATED, Json(post)))

}
async fn add_user(State(pool): State<MySqlPool>, Json(payload): Json<AddUser>)
    -> Result<(StatusCode, Json<User>), StatusCode> {
    let result = sqlx::query(
        "INSERT INTO users(name) VALUES (?)"
    ).bind(&payload.name)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Error with post values: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let new_user_id = result.last_insert_id();

    let user = User {
        id: new_user_id as i32,
        name: Some(payload.name)
    };

    Ok((StatusCode::CREATED, Json(user)))
}
async fn get_users(State(pool): State<MySqlPool>) -> Json<Vec<User>> {
    let result = sqlx::query_as!(
        User,
        "SELECT * FROM users"
    ).fetch_all(&pool).await;

    match result {
        Ok(users) => {
            tracing::info!("Success to get `users` table");
            Json(users)
        }
        Err(e) => {
            tracing::error!("Failed to select users: {e}");
            Json(vec![])
        }
    }
}

async fn get_user_by_id(State(pool): State<MySqlPool>,
    Path(id): Path<i32>
) -> Json<Option<User>>
{
    let result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = ?",
        id
    ).fetch_optional(&pool).await;
    match result {
        Ok(user) => {
            tracing::info!("Succed to get `user` from `users` by id");
            Json(user)
        }
        Err(e) => {
            tracing::error!("Failed to select user: {e}");
            Json(None)
        }
    }
}

async fn get_posts(State(pool): State<MySqlPool>) -> Json<Vec<Post>> {
    let result = sqlx::query_as!(
        Post,
        "SELECT * FROM posts"
    ).fetch_all(&pool).await;

    match result {
        Ok(posts) => {
            tracing::info!("Success to get `posts`");
            Json(posts)
        },
        Err(e) => {
            tracing::error!("Failed to select posts: {e}");
            Json(vec![])
        }
    }
}