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

#[derive(Debug)]
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

#[derive(Debug, Deserialize)]
pub struct UserName {
  pub username: String,
}

#[derive(Debug, Serialize)]
pub struct UserId {
  pub user_id: i32,
}