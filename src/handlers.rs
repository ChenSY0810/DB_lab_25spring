use warp::http::StatusCode;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::models;

use warp::{reject::Reject, Reply};

#[derive(Debug)]
pub struct AuthError;
#[derive(Debug)]
struct DatabaseError(sqlx::Error);

impl Reject for AuthError {} 
impl Reject for DatabaseError {}

pub async fn insert_teacher_handler (
  token: String,
  new_teacher: models::NewTeacher,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {

  // 检查

  if !token.starts_with("Bearer ") || token.len() < 10 {
    return Err(warp::reject::custom(AuthError));
  }

  // 计数
  let cnt = sqlx::query!(
    "SELECT COUNT(*) as count FROM Teacher;"
  )
  .fetch_one(&pool)
  .await
  .map_err(|e| {
    eprintln!("Database error: {}", e);
    warp::reject::custom(DatabaseError(e))
  })?;

  let cnt = format!("{:X}",cnt.count + 1); // 目前教师数量，因为不能使用自增，并且char(5)太短无法应用UUID，我们只能出此下策。

  println!("cur id {}", cnt);

  // 插入
  let result = sqlx::query!(
    "INSERT INTO Teacher (teacher_id, teacher_name, teacher_sex, teacher_title) VALUES (?, ?, ?, ?)",
    cnt,
    new_teacher.teacher_name,
    new_teacher.teacher_sex,
    new_teacher.teacher_title,
  )
  .execute(&pool)
  .await
  .map_err(|e| {
    eprintln!("Database error: {}", e);
    warp::reject::custom(DatabaseError(e))
  });

  // 返回
  match result {
    Ok(_) => Ok(warp::reply::with_status("Inserted", StatusCode::CREATED)),
    Err(e) => {
      eprintln!("Insert error: {:?}", e);
      Ok(warp::reply::with_status("Insert failed", StatusCode::INTERNAL_SERVER_ERROR))
    }
  }

}

pub async fn list_teacher_handler (
  token: String,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection>  {
   // 检查

  if !token.starts_with("Bearer ") || token.len() < 10 {
    return Err(warp::reject::custom(AuthError));
  }

  let rows = sqlx::query_as!(
    models::Teacher,
    r#"
    SELECT teacher_id, teacher_name, teacher_sex, teacher_title
    FROM Teacher
    "#
  )
  .fetch_all(&pool)
  .await
  .map_err(|e| {
    eprintln!("DB error: {:?}", e);
    warp::reject::custom(DatabaseError(e))
  })?;

  Ok(warp::reply::json(&rows))
}