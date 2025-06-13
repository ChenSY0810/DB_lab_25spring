use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Cnt {
  pub count: i64,
}

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

pub trait ToErrorMessage {
    fn to_err_msg(self) -> ErrorMessage;
}

impl ToErrorMessage for &str {
    fn to_err_msg(self) -> ErrorMessage {
        ErrorMessage {
            message: self.to_string(),
        }
    }
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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct UpdateProject {
  pub old_name: String,
  pub new_project: InsertProject,
}

#[derive(Debug, Deserialize)]
pub struct ProjectName {
  pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Project {
  pub project_id: String,
  pub project_name: String,
  pub project_src: String,
  pub project_type: i32,
  pub start_year: i32,
  pub end_year: Option<i32>, 
  pub secret_level: i32,
  pub total_fund: f64,
}

#[derive(Debug, Serialize)]
pub struct ProjectWithTeacher {
  pub project: Project,
  pub teachers: Vec<TeacherRespProject>,
}

#[derive(Debug, Serialize)]
pub struct TeacherRespProject {
  pub teacher_id: String,
  pub teacher_name: String,
  pub fund: f64,
}

#[derive(Debug)]
pub struct TeacherId {
  pub teacher_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertPaper {
  pub name: String,
  pub source: String,
  pub pub_year: u32,
  pub paper_type: i32,
  pub paper_level: i32,
  pub teachers: Vec<TeacherPaperInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeacherPaperInfo {
  pub id: String,      
  pub comm: bool,        
}

#[derive(Debug, Deserialize)]
pub struct PaperName {
  pub name: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Paper {
  pub paper_id: i32,
  pub paper_name: String,
  pub paper_src: String,
  pub paper_type: i32,
  pub paper_level: i32,
  pub pub_year: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PaperWithTeacher {
  pub paper: Paper,
  pub teachers: Vec<TeacherPaper>,
}

#[derive(Debug, Serialize)]
pub struct TeacherPaper {
  pub teacher_id: String,
  pub teacher_name: String,
  pub comm: i8, // 由于返回的是i8 (其实应该是boolean)
}

#[derive(Debug, Deserialize)]
pub struct UpdatePaper {
  pub old_name: String,
  pub new_paper: InsertPaper,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertCourse {
  pub name: String,
  pub course_property: i32,
  pub teachers: Vec<TeacherCourseInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeacherCourseInfo {
  pub id: String,    
  pub year: i32,
  pub semester: i32,
  pub hours: i32,     
}

#[derive(Debug, Deserialize)]
pub struct CourseName {
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Course {
  pub course_id: String,
  pub course_name: String,
  pub course_property: i32,
  pub hours: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseWithTeacher {
  pub course: Course,
  pub teachers: Vec<TeacherCourseInfo>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourse {
  pub old_name: String,
  pub new_course: InsertCourse,
}

#[derive(Debug, Deserialize)]
pub struct RangeQuery {
  pub teacher_id: String,
  pub start_year: Option<i32>,
  pub end_year: Option<i32>,
}

#[derive(Debug)]
pub struct PaperR {
  pub paper_name: String,
  pub paper_src: String,
  pub pub_year: Option<String>,
  pub paper_type: i32,
  pub paper_level: i32,
  pub ranking: i32,
  pub comm_auth: i8,
}

#[derive(Debug)]
pub struct ProjectR {
  pub project_id: String,
  pub project_name: String,
  pub project_src: String,
  pub project_type: i32,
  pub start_year: i32,
  pub end_year: Option<i32>,
  pub secret_level: i32,
  pub ranking: i32,
  pub fund: f64,
}

#[derive(Debug)]
pub struct CourseR {
  pub course_name: String,
  pub course_property: i32,
  pub course_year: i32,
  pub course_semester: i32,
  pub resp_hour: i32,
}