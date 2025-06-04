use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use sqlx::FromRow;
use std::env;
use dotenvy::dotenv;
use chrono::NaiveDate;

pub async fn init_pool() -> MySqlPool {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    
  MySqlPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to connect to MySQL.")

}

// 这里是sql的定义

#[derive(Debug, FromRow)]
pub struct ClassTeach {
  pub teacher_id: String,
  pub course_id: String,
  pub course_year: i32,
  pub course_semester: i32,
  pub resp_hour: i32,
}

#[derive(Debug, FromRow)]
pub struct Course {
  pub course_id: String,
  pub course_name: String,
  pub course_property: i32,
  pub hours: i32,
}

#[derive(Debug, FromRow)]
pub struct PaperPub {
  pub teacher_id: String,
  pub paper_id: i32,
  pub ranking: i32,
  pub comm_auth: bool,
}

#[derive(Debug, FromRow)]
pub struct Project {
  pub project_id: String,
  pub project_name: String,
  pub project_src: String,
  pub project_type: i32,
  pub total_fund: f32,
  pub start_year: i32,
  pub end_year: Option<i32>,
}

#[derive(Debug, FromRow)]
pub struct ProjectResp {
  pub teacher_id: String,
  pub project_id: String,
  pub ranking: i32,
  pub fund: f32,
}


#[derive(Debug, FromRow)]
pub struct Publication {
  pub paper_id: i32,
  pub paper_name: String,
  pub paper_src: String,
  pub pub_year: NaiveDate,
  pub paper_type: i32,
  pub paper_level: i32,
}

#[derive(Debug, FromRow)]
pub struct Teacher {
  pub teacher_id: String,
  pub teacher_name: String,
  pub teacher_sex: i32,
  pub teacher_title: i32,
}
