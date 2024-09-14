use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub description: String,
    pub project_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DbTask {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub description: String,
    pub project_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub author_id: i32,
    pub name: String,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
pub struct DbProject {
    pub id: i32,
    pub author_id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct DbUser {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}


#[derive(Serialize, Deserialize)]
pub struct UserProject {
    pub user_id: Option<i32>,
    pub project_id: Option<i32>,
}