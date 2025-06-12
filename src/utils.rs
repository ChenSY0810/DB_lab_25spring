use warp::http::StatusCode;
use sqlx::MySqlPool;
use crate::models::*;
use warp::{reject::Reject};
use serde;

#[derive(Debug)]
pub struct HashError;
#[derive(Debug)]
pub struct DatabaseError(sqlx::Error);

impl Reject for HashError {}
impl Reject for DatabaseError {}


pub trait DbResultExt<T> {
    fn map_db_err(self) -> Result<T, warp::Rejection>;
}

impl<T> DbResultExt<T> for Result<T, sqlx::Error> {
    fn map_db_err(self) -> Result<T, warp::Rejection> {
        self.map_err(|e| {
            eprintln!("DB error: {:?}", e);
            warp::reject::custom(DatabaseError(e))
        })
    }
}

pub fn ret_format<T: serde::Serialize + Send + Sync + 'static>(
    data: T,
    status: StatusCode,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let json = warp::reply::json(&data);
    let with_status = warp::reply::with_status(json, status);
    Ok(Box::new(with_status))
}

pub fn formalize_teacher_id (
  old_id: &str,
) -> String {
  format!("{:X}", i32::from_str_radix(old_id, 16).unwrap())
}

pub async fn get_user_teacher_id (
  username: &String,
  pool: &MySqlPool,
) -> Option<String> {

  sqlx::query_as!(
    TeacherId,
    r#"
    SELECT teacher_id
    FROM User
    WHERE user_name = ?
    "#,
    username
  )
  .fetch_one(pool).await
  .map_db_err()
  .unwrap()
  .teacher_id

}

pub async fn get_user_priv (
  username: &String,
  pool: &MySqlPool,
) -> i32 {
  sqlx::query_as!(
    Privilege,
    r#"
    SELECT user_privilege as privilege
    FROM User
    WHERE user_name = ?
    "#,
    username
  )
  .fetch_one(pool).await
  .map_db_err().unwrap()
  .privilege
}
