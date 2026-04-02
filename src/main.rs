use axum::{
    // Работа с роуторами
    extract::{Form, Path, State},
    http::StatusCode,
    routing::{get, post},
    // Работа с get, формами
    Json,
    // enum с статус кодами ответа
    Router
};
// Используется для работы с JSON
use serde_json::json;
use serde::Serialize;
// Подключение к БД
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use api_blog_ru::models::post_model::*;
use api_blog_ru::models::user_model::*;

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