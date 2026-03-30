use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub author_id: i32,
}

#[derive(Deserialize)]
pub struct AddUser {
    pub name: String
}

#[derive(Deserialize)]
pub struct AddPost {
    // pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub author_id: i32
}