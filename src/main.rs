mod tables;

use tables::User;
use axum::{
    routing::get,
    Router,
    Json,
    extract::{State, Path}
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
        .with_state(pool);

    let url = "127.0.0.1:3030";
    let listener =
        tokio::net::TcpListener::bind(url).await.unwrap();

    tracing::info!("Open server in http://{url}");
    axum::serve(listener, api_app).await.unwrap();
}

async fn get_users(State(pool): State<MySqlPool>) -> Json<Vec<User>> {
    let result = sqlx::query_as!(
        User,
        "SELECT * FROM users"
    ).fetch_all(&pool).await;

    match result {
        Ok(users) => Json(users),
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
        Ok(user) => Json(user),
        Err(e) => {
            tracing::error!("Failed to select user: {e}");
            Json(None)
        }
    }
}