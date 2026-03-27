use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub author_id: i32,
}