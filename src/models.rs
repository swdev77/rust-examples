use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub title: String,
    pub username: String,
}

