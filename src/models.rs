use serde::{Deserialize, Serialize};

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
