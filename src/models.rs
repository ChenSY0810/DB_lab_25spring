use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewTeacher {
  pub teacher_name: String,
  pub teacher_sex: i32,
  pub teacher_title: i32,
}