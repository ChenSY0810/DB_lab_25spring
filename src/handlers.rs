use warp::http::StatusCode;
use sqlx::MySqlPool;
use uuid::Uuid;
use warp::{reject::Reject};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::models::{self};


#[derive(Debug)]
pub struct HashError;
#[derive(Debug)]
struct DatabaseError(sqlx::Error);

impl Reject for HashError {}
impl Reject for DatabaseError {}

fn formalize_teacher_id (
  old_id: &str,
) -> String {
  format!("{:X}", i32::from_str_radix(old_id, 16).unwrap())
}

pub async fn insert_teacher_handler (
  new_teacher: models::NewTeacher,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {

  // 计数
  let cnt = sqlx::query!(
    r#"
    SELECT COUNT(*) as count 
    FROM Teacher
    "#
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
    r#"
    INSERT INTO Teacher 
    (teacher_id, teacher_name, teacher_sex, teacher_title) 
    VALUES (?, ?, ?, ?)
    "#,
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
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection>  {

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

pub async fn list_user_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {

  let rows = sqlx::query_as!(
    models::UserName,
    r#"
    SELECT user_name as username
    FROM User
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

pub async fn register_handler (
  login_data: models::LoginData,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection>  {

  let hashed = hash(login_data.password, DEFAULT_COST)
  .map_err(|_| warp::reject::custom(HashError))?;
  let result = sqlx::query!(
    r#"
    INSERT INTO User
    (user_name, user_password)
    VALUE (?, ?)
    "#, 
    login_data.username, hashed
  )
  .execute(&pool)
  .await
  .map_err(|e| {
    eprintln!("DB error: {:?}", e);
    warp::reject::custom(DatabaseError(e))
  });

  match result {
    Ok(_) => Ok(warp::reply::with_status("Registered.", StatusCode::CREATED)),
    Err(e) => {
      eprintln!("Registration error: {:?}", e);
      Ok(warp::reply::with_status("Registration failed", StatusCode::INTERNAL_SERVER_ERROR))
    }
  }
}

pub async fn login_handler (
  login_data: models::LoginData,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection>  {

  let pwd = sqlx::query_as!(
    models::Password,
    r#"
    SELECT user_password as password 
    FROM User
    WHERE user_name = ?
    "#, 
    login_data.username,
  )
  .fetch_optional(&pool)
  .await
  .map_err(|e| {
    eprintln!("DB error: {:?}", e);
    warp::reject::custom(DatabaseError(e))
  })?;

  let Some(pwd) = pwd else {
    let err = models::ErrorMessage {
      message: "User not found".into(),
    };
    return Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::UNAUTHORIZED));
  };

  let valid = verify(login_data.password, &pwd.password).unwrap_or(false);

  if !valid {
    let err = models::ErrorMessage {
      message: "Wrong password".into(),
    };
    return Ok(warp::reply::with_status(warp::reply::json(&err), StatusCode::UNAUTHORIZED));
  }

  let privilege = sqlx::query_as!(
    models::Privilege,
    r#"
    SELECT user_privilege as privilege 
    FROM User
    WHERE user_name = ?
    "#, 
    login_data.username
  )
  .fetch_one(&pool)
  .await
  .map_err(|e| {
    eprintln!("DB error: {:?}", e);
    warp::reject::custom(DatabaseError(e))
  })?;

  Ok(warp::reply::with_status(warp::reply::json(&privilege), StatusCode::OK))
}

pub async fn get_user_id_handler (
  username: models::UserName,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection>  {

  let id = sqlx::query_as!(
    models::UserId,
    r#"
    SELECT user_id 
    FROM User
    WHERE user_name = ?
    "#, 
    username.username
  )
  .fetch_one(&pool)
  .await
  .map_err(|e| {
    eprintln!("DB error: {:?}", e);
    warp::reject::custom(DatabaseError(e))
  })?;

  Ok(warp::reply::json(&id))
}

pub async fn link_handler (
  link: models::Link,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {

  // 正规化
  let id = formalize_teacher_id(&link.teacher_id);

  let result = sqlx::query!(
    r#"
    UPDATE User
    SET teacher_id = ?
    WHERE user_name = ?
    "#, 
    id, link.user_name,
  )
  .execute(&pool)
  .await
  .map_err(|e| {
    eprintln!("DB error: {:?}", e);
    warp::reject::custom(DatabaseError(e))
  });

  match result {
    Ok(_) => Ok(warp::reply::with_status("Linked.", StatusCode::OK)),
    Err(e) => {
      eprintln!("Link error: {:?}", e);
      Ok(warp::reply::with_status("Link failed", StatusCode::INTERNAL_SERVER_ERROR))
    }
  }
}

pub async fn project_create_handler (
  project: models::InsertProject,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {

  // 检查是否存在重复教师
  if project.teachers.len() != 0 {
    use std::collections::HashSet;
    let ids: Vec<_> = project.teachers.iter().map(|t| i32::from_str_radix(&t.id, 16).unwrap()).collect();
    let unique_ids: HashSet<_> = ids.iter().copied().collect();
    if ids.len() != unique_ids.len() {
      return Ok(warp::reply::with_status("A teacher has multiple rank.", StatusCode::UNPROCESSABLE_ENTITY));
    }
  }

  // 插入project
  let id = Uuid::new_v4().to_string(); // 其实就是一个随机数
  sqlx::query!(
    r#"
    INSERT INTO Project
    (project_id, project_name, project_src, project_type, start_year, secret_level)
    VALUE (?, ?, ?, ?, ?, ?)
    "#,
    id, project.name, project.source, project.project_type, project.start_year, project.secret_level
  )
  .execute(&pool)
  .await
  .map_err(|e| {
    eprintln!("DB error: {:?}", e);
    warp::reject::custom(DatabaseError(e))
  })?;

  if let Some(end) = project.end_year {
    sqlx::query!(
      r#"
      UPDATE Project
      SET end_year = ?
      WHERE project_id = ?
      "#,
      end, id,
    ).execute(&pool)
    .await
    .map_err(|e| {
      eprintln!("DB error: {:?}", e);
      warp::reject::custom(DatabaseError(e))
    })?;
  }

  // 插入每一个教师
  let mut idx = 1;
  for teacher in project.teachers {
    // 正规化
    let teacher_id = formalize_teacher_id(&teacher.id);
    if teacher.fund < 0.0 {
      return Ok(warp::reply::with_status("Contribute Negative.", StatusCode::UNPROCESSABLE_ENTITY));
    }
    sqlx::query!(
      r#"
      INSERT INTO ProjectResp
      (teacher_id, project_id, ranking, fund)
      VALUE (?, ?, ?, ?)
      "#,
      teacher_id, id, idx, teacher.fund
    )
    .execute(&pool)
    .await
    .map_err(|e| {
      eprintln!("DB error: {:?}", e);
      warp::reject::custom(DatabaseError(e))
    })?;

    idx = idx + 1;
  }

  Ok(warp::reply::with_status("Success.", StatusCode::OK))
}

pub async fn project_read_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn project_update_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn project_delete_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn paper_create_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn paper_read_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn paper_update_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn paper_delete_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn course_create_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn course_read_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn course_update_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn course_delete_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn range_query_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn query_pdf_handler (
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn change_password_handler (
  username: models::UserName,
  pwd: models::Password,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection>  {

  let hashed = hash(pwd.password, DEFAULT_COST)
  .map_err(|_| warp::reject::custom(HashError))?;

  let result = sqlx::query!(
    r#"
    UPDATE User 
    SET user_password = ?
    WHERE user_name = ?;
    "#, 
    hashed, username.username
  )
  .execute(&pool)
  .await
  .map_err(|e| {
    eprintln!("DB error: {:?}", e);
    warp::reject::custom(DatabaseError(e))
  });

  match result {
    Ok(_) => Ok(warp::reply::with_status("Changed", StatusCode::OK)),
    Err(e) => {
      eprintln!("Insert error: {:?}", e);
      Ok(warp::reply::with_status("Change Password Failed.", StatusCode::INTERNAL_SERVER_ERROR))
    }
  }
}