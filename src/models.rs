use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginData {
  pub username: String,
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Privilege {
  pub privilege: i32,
}

#[derive(Debug, Deserialize)]
pub struct Password {
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
  pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct NewTeacher {
  pub teacher_name: String,
  pub teacher_sex: i32,
  pub teacher_title: i32,
}
#[derive(Debug, Serialize)]
pub struct Teacher {
  pub teacher_id: String,
  pub teacher_name: String,
  pub teacher_sex: i32,
  pub teacher_title: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserName {
  pub username: String,
}

#[derive(Debug, Serialize)]
pub struct UserId {
  pub user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct Link {
  pub teacher_id: String,
  pub user_name: String,
}

#[derive(Debug, Deserialize)]
pub struct TeacherFundInfo {
  pub id: String,      
  pub fund: f64,        
}

#[derive(Debug, Deserialize)]
pub struct InsertProject {
  pub name: String,
  pub source: String,
  pub project_type: i32,
  pub start_year: i32,
  pub end_year: Option<i32>, 
  pub secret_level: i32,
  pub teachers: Vec<TeacherFundInfo>, 
}