use tokio::fs;
use warp::http::StatusCode;
use sqlx::MySqlPool;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::fs::File;
use std::path::PathBuf;
use std::io::Write as IoWrite;
use std::fmt::Write as FmtWrite;
use crate::models::{self, ToErrorMessage};
use crate::utils::*;


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
  .fetch_one(&pool).await
  .map_db_err()?;

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
  .execute(&pool).await
  .map_db_err();

  // 返回
  match result {
    Ok(_) => ret_format("Inserted", StatusCode::CREATED),
    Err(e) => {
      eprintln!("Insert error: {:?}", e);
      ret_format("Insert failed", StatusCode::INTERNAL_SERVER_ERROR)
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
  .fetch_all(&pool).await
  .map_db_err()?;

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
  .fetch_all(&pool).await
  .map_db_err()?;

  ret_format(rows, StatusCode::OK)
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
  .execute(&pool).await
  .map_db_err();

  match result {
    Ok(_) => ret_format("Registered", StatusCode::CREATED),
    Err(e) => {
      eprintln!("Registration error: {:?}", e);
      ret_format("Registration failed", StatusCode::INTERNAL_SERVER_ERROR)
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
  .fetch_optional(&pool).await
  .map_db_err()?;

  let Some(pwd) = pwd else {
    return ret_format("User Not Found".to_err_msg(), StatusCode::UNAUTHORIZED);
  };

  let valid = verify(login_data.password, &pwd.password).unwrap_or(false);

  if !valid {
    return ret_format("Wrong password".to_err_msg(), StatusCode::UNAUTHORIZED);
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
  .fetch_one(&pool).await
  .map_db_err()?;

  ret_format(privilege, StatusCode::OK)
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
  .fetch_one(&pool).await
  .map_db_err()?;

  ret_format(id, StatusCode::OK)
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
  .execute(&pool).await
  .map_db_err();

  match result {
    Ok(_) => ret_format("Linked.".to_err_msg(), StatusCode::OK),
    Err(e) => {
      eprintln!("Link error: {:?}", e);
      ret_format("Link failed".to_err_msg(), StatusCode::FORBIDDEN)
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
      return ret_format("A teacher has multiple rank.".to_err_msg(), StatusCode::UNPROCESSABLE_ENTITY);
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
  .execute(&pool).await
  .map_db_err()?;

  if let Some(end) = project.end_year {
    sqlx::query!(
      r#"
      UPDATE Project
      SET end_year = ?
      WHERE project_id = ?
      "#,
      end, id,
    ).execute(&pool).await
    .map_db_err()?;
  }

  // 插入每一个教师
  let mut idx = 1;
  for teacher in project.teachers {
    // 正规化
    let teacher_id = formalize_teacher_id(&teacher.id);
    if teacher.fund < 0.0 {
      return ret_format("Contribute Negative.", StatusCode::UNPROCESSABLE_ENTITY);
    }
    sqlx::query!(
      r#"
      INSERT INTO ProjectResp
      (teacher_id, project_id, ranking, fund)
      VALUE (?, ?, ?, ?)
      "#,
      teacher_id, id, idx, teacher.fund
    )
    .execute(&pool).await
    .map_db_err()?;

    idx = idx + 1;
  }
  ret_format("Success.".to_err_msg(), StatusCode::OK)
}

pub async fn project_read_handler (
  user_name: models::UserName,
  project_name: models::ProjectName,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {

  // 查询，检查权限
  let project = sqlx::query_as!(
    models::Project,
    r#"
    SELECT *
    FROM Project
    WHERE project_name = ?
    "#,
    project_name.name
  )
  .fetch_one(&pool).await
  .map_db_err()?;

  let pr = sqlx::query_as!(
    models::TeacherRespProject,
    r#"
    SELECT t.teacher_id, t.teacher_name, pr.fund
    FROM Teacher t
    INNER JOIN ProjectResp pr ON pr.teacher_id = t.teacher_id
    WHERE pr.project_id = ?
    "#,
    project.project_id,
  )
  .fetch_all(&pool).await
  .map_db_err()?;

  let ret = models::ProjectWithTeacher {
    project: project, 
    teachers: pr,
  };

  if ret.project.secret_level == 2 {
    // admin or in projectresp
    let pri = get_user_priv(&user_name.username, &pool).await;
    
    let emsg = "No Permission(Make sure you are admin or the project's participant.)".to_err_msg();

    if pri == 1 {
      let tid = get_user_teacher_id(&user_name.username, &pool).await;
      match tid {
        Some(str) => {
          let cnt = sqlx::query_as!(
            models::Cnt,
            r#"
            SELECT COUNT(1) as count
            FROM ProjectResp
            WHERE teacher_id = ?
            AND project_id = ?
            "#,
            str, ret.project.project_id
          )
          .fetch_one(&pool).await
          .map_db_err()?;

          if cnt.count == 1 {
            ret_format(ret, StatusCode::OK)
          } else {
            ret_format(emsg, StatusCode::FORBIDDEN)
          }

        }, 
        _ => ret_format(emsg, StatusCode::FORBIDDEN)
      }
    } else {
      ret_format(ret, StatusCode::OK)
    }
  } else {
    ret_format(ret, StatusCode::OK)
  }
}

pub async fn project_update_handler (
  user_name: models::UserName,
  project: models::UpdateProject,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {

  let pri = get_user_priv(&user_name.username, &pool).await;

  let pid = sqlx::query_as!(
    models::UserName,  // 借用一下
    r#"
    SELECT project_id as username
    FROM Project
    WHERE project_name = ?
    "#,
    project.old_name
  )
  .fetch_one(&pool).await
  .map_db_err()?
  .username;

  if pri == 1 {

    let tid = get_user_teacher_id(&user_name.username, &pool).await.unwrap();

    let cnt = sqlx::query_as!(
      models::Cnt,
      r#"
      SELECT COUNT(1) as count
      FROM ProjectResp
      WHERE teacher_id = ?
      AND project_id = ?
      AND ranking = 1
      "#,
      tid, pid
    )
    .fetch_one(&pool).await
    .map_db_err()?;

    if cnt.count != 1 {
      let emsg = "No Permission(Make sure you are admin or the project's leader.)".to_err_msg();
      return ret_format(emsg, StatusCode::FORBIDDEN);
    }
  }

  let result = sqlx::query!(
    r#"
      UPDATE Project
      SET project_name = ?, project_src = ?, project_type = ?, start_year = ?, end_year = ?, secret_level = ?
      WHERE project_name = ?
    "#,
    project.new_project.name, 
    project.new_project.source, 
    project.new_project.project_type, 
    project.new_project.start_year, 
    project.new_project.end_year, 
    project.new_project.secret_level, 
    project.old_name
  )
  .execute(&pool).await
  .map_db_err();
  
  match result {
    Ok(_) => {
      sqlx::query!(
        r#"
        DELETE 
        FROM ProjectResp
        WHERE project_id = ?
        "#,
        pid
      )
      .execute(&pool)
      .await
      .map_db_err()?;

      for (idx, teacher) in project.new_project.teachers.iter().enumerate() {
        // 正规化
        let tid = formalize_teacher_id(&teacher.id);
        if teacher.fund < 0.0 {
          return ret_format("Contribute Negative.", StatusCode::UNPROCESSABLE_ENTITY);
        }
        sqlx::query!(
          r#"
          INSERT INTO ProjectResp
          (teacher_id, project_id, ranking, fund)
          VALUE (?, ?, ?, ?)
          "#,
          tid, pid, (idx + 1) as i32, teacher.fund
        )
        .execute(&pool).await
        .map_db_err()?;

      }
      ret_format("Successfull Updated.", StatusCode::OK)
    },
    Err(e) => {
      eprintln!("Update error: {:?}", e);
      ret_format("Update failed", StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}

pub async fn project_delete_handler (
  uname: models::UserName,
  pname: models::ProjectName,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {

  let pri = get_user_priv(&uname.username, &pool).await;

  let pid = sqlx::query_as!(
    models::UserName,  // 借用一下
    r#"
    SELECT project_id as username
    FROM Project
    WHERE project_name = ?
    "#,
    pname.name
  )
  .fetch_one(&pool).await
  .map_db_err()?
  .username;

  if pri == 1 {
    let tid = get_user_teacher_id(&uname.username, &pool).await.unwrap();

    let cnt = sqlx::query_as!(
      models::Cnt,
      r#"
      SELECT COUNT(1) as count
      FROM ProjectResp
      WHERE teacher_id = ?
      AND project_id = ?
      AND ranking = 1
      "#,
      tid, &pid
    )
    .fetch_one(&pool).await
    .map_db_err()?;

    if cnt.count != 1 {
      let emsg = "No Permission(Make sure you are admin or the project's leader.)".to_err_msg();
      return ret_format(emsg, StatusCode::FORBIDDEN);
    }
  } 
  
  let result = sqlx::query!(
    r#"
    DELETE 
    FROM Project
    WHERE project_id = ?
    "#,
    pid
  )
  .execute(&pool).await
  .map_db_err();
  
  match result {
    Ok(_) => ret_format("Deleted.".to_err_msg(), StatusCode::OK),
    Err(e) => {
      eprintln!("Update error: {:?}", e);
      ret_format("Update failed", StatusCode::INTERNAL_SERVER_ERROR)
    }
  }

}

pub async fn paper_create_handler(
  paper: models::InsertPaper,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  // 检查是否存在重复教师
  if !paper.teachers.is_empty() {
    use std::collections::HashSet;
    let ids: Vec<_> = paper.teachers.iter().map(|t| t.id.clone()).collect();
    let unique_ids: HashSet<_> = ids.iter().cloned().collect();
    if ids.len() != unique_ids.len() {
      return ret_format("A teacher has multiple rank.".to_err_msg(), StatusCode::UNPROCESSABLE_ENTITY);
    }
  }

  // 插入论文记录
  let pub_date = format!("{}-01-01", paper.pub_year); 
  let result = sqlx::query!(
    r#"
    INSERT INTO Publication
    (paper_name, paper_src, pub_year, paper_type, paper_level)
    VALUES (?, ?, ?, ?, ?)
    "#,
    paper.name,
    paper.source,
    pub_date,
    paper.paper_type,
    paper.paper_level,
  )
  .execute(&pool)
  .await
  .map_db_err()?;

  // 获取插入后的 paper_id（auto increment）
  let paper_id = result.last_insert_id() as i32;

  // 插入教师参与记录
  for (idx, teacher) in paper.teachers.iter().enumerate() {
    let teacher_id = formalize_teacher_id(&teacher.id);

    sqlx::query!(
      r#"
      INSERT INTO PaperPub
      (teacher_id, paper_id, ranking, comm_auth)
      VALUES (?, ?, ?, ?)
      "#,
      teacher_id,
      paper_id,
      (idx + 1) as i32,
      teacher.comm,
    )
    .execute(&pool)
    .await
    .map_db_err()?;
  }

  ret_format("Success.".to_err_msg(), StatusCode::OK)
}

pub async fn paper_read_handler(
  paper_name: models::PaperName,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  // 查询论文基本信息
  let paper = sqlx::query_as!(
    models::Paper,
    r#"
    SELECT
      paper_id,
      paper_name,
      paper_src,
      paper_type,
      paper_level,
      DATE_FORMAT(pub_year, '%Y') as pub_year
    FROM Publication
    WHERE paper_name = ?
    "#,
    paper_name.name
  )
  .fetch_one(&pool)
  .await
  .map_db_err()?;

  // 查询该论文的教师信息
  let teachers = sqlx::query_as!(
    models::TeacherPaper,
    r#"
    SELECT
      t.teacher_id,
      t.teacher_name,
      p.comm_auth as comm
    FROM Teacher t
    INNER JOIN PaperPub p ON p.teacher_id = t.teacher_id
    WHERE p.paper_id = ?
    "#,
    paper.paper_id
  )
  .fetch_all(&pool)
  .await
  .map_db_err()?;

  // 封装为返回结构
  let ret = models::PaperWithTeacher {
    paper,
    teachers,
  };

  ret_format(ret, StatusCode::OK)
}

pub async fn paper_update_handler(
  user_name: models::UserName,
  paper: models::UpdatePaper,   
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  // 获取用户权限
  let pri = get_user_priv(&user_name.username, &pool).await;

  // 先根据 old_name 查出 paper_id
  let paper_id = sqlx::query_as!(
    models::UserId, // 用 UserName 临时借用存 paper_id
    r#"
    SELECT paper_id as user_id
    FROM Publication
    WHERE paper_name = ?
    "#,
    paper.old_name
  )
  .fetch_one(&pool)
  .await
  .map_db_err()?
  .user_id;

  if pri == 1 {
    // 非admin，需要判断是否是通信作者
    let tid = get_user_teacher_id(&user_name.username, &pool).await;

    match tid {
      Some(tid) => {
        let comm_auth_count = sqlx::query_as!(
          models::Cnt,
          r#"
          SELECT COUNT(1) as count
          FROM PaperPub
          WHERE teacher_id = ?
          AND paper_id = ?
          AND comm_auth = TRUE
          "#,
          tid,
          paper_id
        )
        .fetch_one(&pool)
        .await
        .map_db_err()?;

        if comm_auth_count.count != 1 {
          let emsg = "No Permission (must be admin or comm. author)".to_err_msg();
          return ret_format(emsg, StatusCode::FORBIDDEN);
        }
      }
      None => {
        let emsg = "No Permission (must be admin or comm. author)".to_err_msg();
        return ret_format(emsg, StatusCode::FORBIDDEN);
      }
    }
  }

  let pub_date = format!("{}-01-01", paper.new_paper.pub_year); 
  let result = sqlx::query!(
    r#"
    UPDATE Publication
    SET paper_name = ?, paper_src = ?, paper_type = ?, paper_level = ?, pub_year = ?
    WHERE paper_name = ?
    "#,
    paper.new_paper.name,
    paper.new_paper.source,
    paper.new_paper.paper_type,
    paper.new_paper.paper_level,
    pub_date,
    paper.old_name
  )
  .execute(&pool)
  .await
  .map_db_err();

  match result {
    Ok(_) => {
      sqlx::query!(
        r#"
        DELETE FROM PaperPub
        WHERE paper_id = ?
        "#,
        paper_id
      )
      .execute(&pool)
      .await
      .map_db_err()?;

      for (idx, teacher) in paper.new_paper.teachers.iter().enumerate() {
        let tid = formalize_teacher_id(&teacher.id);

        sqlx::query!(
            r#"
            INSERT INTO PaperPub
            (teacher_id, paper_id, ranking, comm_auth)
            VALUES (?, ?, ?, ?)
            "#,
            tid,
            paper_id,
            (idx + 1) as i32,
            teacher.comm
        )
        .execute(&pool)
        .await
        .map_db_err()?;
      }

      ret_format("Successfully updated paper.".to_err_msg(), StatusCode::OK)
    }
    Err(e) => {
        eprintln!("Update error: {:?}", e);
        ret_format("Update failed".to_err_msg(), StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}

pub async fn paper_delete_handler(
  uname: models::UserName,
  pname: models::PaperName,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  // 获取权限
  let pri = get_user_priv(&uname.username, &pool).await;

  let paper_id = sqlx::query_as!(
    models::UserId, // 复用结构体
    r#"
    SELECT paper_id as user_id
    FROM Publication
    WHERE paper_name = ?
    "#,
    pname.name
  )
  .fetch_one(&pool).await
  .map_db_err()?
  .user_id;

  if pri == 1 {
    let tid = get_user_teacher_id(&uname.username, &pool).await.unwrap();

    let cnt = sqlx::query_as!(
      models::Cnt,
      r#"
      SELECT COUNT(1) as count
      FROM PaperPub
      WHERE teacher_id = ?
      AND paper_id = ?
      AND comm_auth = TRUE
      "#,
      tid, &paper_id
    )
    .fetch_one(&pool).await
    .map_db_err()?;

    if cnt.count != 1 {
      let emsg = "No Permission (Only admin or the paper's comm. author can delete.)".to_err_msg();
      return ret_format(emsg, StatusCode::FORBIDDEN);
    }
  }

  let result = sqlx::query!(
    r#"
    DELETE FROM Publication
    WHERE paper_id = ?
    "#,
    paper_id
  )
  .execute(&pool).await
  .map_db_err();

  match result {
    Ok(_) => ret_format("Deleted.".to_err_msg(), StatusCode::OK),
    Err(e) => {
      eprintln!("Delete error: {:?}", e);
      ret_format("Delete failed", StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}

pub async fn course_create_handler(
  course: models::InsertCourse,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  use std::collections::HashSet;

  // 检查教师是否在同一时间重复
  if !course.teachers.is_empty() {
    let mut set = HashSet::new();
    for t in &course.teachers {
      let norm_id = formalize_teacher_id(&t.id);
      let key = format!("{}-{}-{}", norm_id, t.year, t.semester);
      if !set.insert(key) {
        return ret_format("A teacher has duplicate records in the same term.".to_err_msg(), StatusCode::UNPROCESSABLE_ENTITY);
      }
    }
  }

  // 使用uuid
  let cid = uuid::Uuid::new_v4().to_string();

  sqlx::query!(
    r#"
    INSERT INTO Course (course_id, course_name, course_property)
    VALUES (?, ?, ?)
    "#,
    cid,
    course.name,
    course.course_property
  )
  .execute(&pool)
  .await
  .map_db_err()?;

  for teacher in &course.teachers {
    if teacher.hours == 0 {
      return ret_format("Hours must be positive.".to_err_msg(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    let tid = formalize_teacher_id(&teacher.id);

    sqlx::query!(
      r#"
      INSERT INTO ClassTeach (teacher_id, course_id, course_year, course_semester, resp_hour)
      VALUES (?, ?, ?, ?, ?)
      "#,
      tid,
      cid,
      teacher.year as i32,
      teacher.semester as i32,
      teacher.hours as i32
    )
    .execute(&pool)
    .await
    .map_db_err()?;
  }

  ret_format("Success.".to_err_msg(), StatusCode::OK)
}

pub async fn course_read_handler(
  course_name: models::CourseName,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  // 查询课程基本信息
  let course = sqlx::query_as!(
    models::Course,
    r#"
    SELECT
      course_id,
      course_name,
      course_property,
      hours
    FROM Course
    WHERE course_name = ?
    "#,
    course_name.name
  )
  .fetch_one(&pool)
  .await
  .map_db_err()?;

  // 查询主讲教师信息
  let teachers = sqlx::query_as!(
    models::TeacherCourseInfo,
    r#"
    SELECT
      t.teacher_id as id,
      c.course_year as year,
      c.course_semester as semester,
      c.resp_hour as hours
    FROM Teacher t
    INNER JOIN ClassTeach c ON c.teacher_id = t.teacher_id
    WHERE c.course_id = ?
    "#,
    course.course_id
  )
  .fetch_all(&pool)
  .await
  .map_db_err()?;

  // 封装为返回结构
  let ret = models::CourseWithTeacher {
    course,
    teachers,
  };

  ret_format(ret, StatusCode::OK)
}

pub async fn course_update_handler(
  user_name: models::UserName,
  course: models::UpdateCourse,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  let pri = get_user_priv(&user_name.username, &pool).await;

  // 先查 course_id
  let course_id = sqlx::query_as!(
    models::UserName, // 复用结构体
    r#"
    SELECT course_id as username
    FROM Course
    WHERE course_name = ?
    "#,
    course.old_name
  )
  .fetch_one(&pool)
  .await
  .map_db_err()?
  .username;

  if pri == 1 {
    // 非管理员，要检查是否是负责教师
    let tid = get_user_teacher_id(&user_name.username, &pool).await;
    match tid {
      Some(tid) => {
        let cnt = sqlx::query_as!(
          models::Cnt,
          r#"
          SELECT COUNT(1) as count
          FROM ClassTeach
          WHERE teacher_id = ?
            AND course_id = ?
          "#,
          tid,
          course_id
        )
        .fetch_one(&pool)
        .await
        .map_db_err()?;

        if cnt.count == 0 {
          return ret_format("No Permission (must be admin or course teacher)".to_err_msg(), StatusCode::FORBIDDEN);
        }
      }
      None => {
        return ret_format("No Permission (must be admin or course teacher)".to_err_msg(), StatusCode::FORBIDDEN);
      }
    }
  }

  let result = sqlx::query!(
    r#"
    UPDATE Course
    SET course_name = ?, course_property = ?
    WHERE course_name = ?
    "#,
    course.new_course.name,
    course.new_course.course_property,
    course.old_name
  )
  .execute(&pool)
  .await
  .map_db_err();

  match result {
    Ok(_) => {
      // 先删除旧教师关系
      sqlx::query!(
        r#"
        DELETE FROM ClassTeach
        WHERE course_id = ?
        "#,
        course_id
      )
      .execute(&pool)
      .await
      .map_db_err()?;

      // 插入新的教师关系
      for teacher in &course.new_course.teachers {
        let tid = formalize_teacher_id(&teacher.id);

        sqlx::query!(
          r#"
          INSERT INTO ClassTeach
          (teacher_id, course_id, resp_hour, course_year, course_semester)
          VALUES (?, ?, ?, ?, ?)
          "#,
          tid,
          course_id,
          teacher.hours,
          teacher.year,
          teacher.semester
        )
        .execute(&pool)
        .await
        .map_db_err()?;
      }

      ret_format("Successfully updated course.".to_err_msg(), StatusCode::OK)
    }
    Err(e) => {
      eprintln!("Update error: {:?}", e);
      ret_format("Update failed".to_err_msg(), StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}


pub async fn course_delete_handler(
  uname: models::UserName,
  cname: models::CourseName,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
  // 获取用户权限
  let pri = get_user_priv(&uname.username, &pool).await;

  // 获取 course_id
  let course_id = sqlx::query_as!(
    models::UserName, // 复用结构体
    r#"
    SELECT course_id as username
    FROM Course
    WHERE course_name = ?
    "#,
    cname.name
  )
  .fetch_one(&pool)
  .await
  .map_db_err()?
  .username;

  // 如果不是管理员，需要检查是否为该课程任课教师
  if pri == 1 {
    let tid_opt = get_user_teacher_id(&uname.username, &pool).await;

    if let Some(tid) = tid_opt {
      let cnt = sqlx::query_as!(
        models::Cnt,
        r#"
        SELECT COUNT(1) as count
        FROM ClassTeach
        WHERE teacher_id = ?
        AND course_id = ?
        "#,
        tid,
        course_id
      )
      .fetch_one(&pool)
      .await
      .map_db_err()?;

      if cnt.count == 0 {
        let emsg = "No Permission (Only admin or course teacher can delete.)".to_err_msg();
        return ret_format(emsg, StatusCode::FORBIDDEN);
      }
    } else {
      let emsg = "No Permission (Only admin or course teacher can delete.)".to_err_msg();
      return ret_format(emsg, StatusCode::FORBIDDEN);
    }
  }

  // 执行删除
  let result = sqlx::query!(
    r#"
    DELETE FROM Course
    WHERE course_id = ?
    "#,
    course_id
  )
  .execute(&pool)
  .await
  .map_db_err();

  match result {
    Ok(_) => ret_format("Deleted.".to_err_msg(), StatusCode::OK),
    Err(e) => {
      eprintln!("Delete error: {:?}", e);
      ret_format("Delete failed", StatusCode::INTERNAL_SERVER_ERROR)
    }
  }
}

pub async fn range_query_handler(
  user_name: models::UserName,
  query: models::RangeQuery,
  pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {

  // 写入， typst编译， 发送

  let pri = get_user_priv(&user_name.username, &pool).await;
  let mut not_teacher = false;
  let tid=  query.teacher_id;
  let utid = match get_user_teacher_id(&user_name.username, &pool).await {
    Some(a) => a,
    None => {
      if pri == 2 {
        not_teacher = true;
      }
      "".to_string()
    }
  };
  let start_year = match query.start_year {
    Some(a) => a,
    None => 0, 
  };
  let end_year = match query.end_year {
    Some(a) => a,
    None => 10000, // 这只是一个充分大的数
  };


  // teacher paper project course
  let teacher = sqlx::query_as!(
    models::Teacher,
    r#"
    SELECT *
    FROM Teacher
    WHERE teacher_id = ?
    "#,
    &tid
  )
  .fetch_one(&pool)
  .await
  .map_db_err()?;

  let papers = sqlx::query_as!(
    models::PaperR,
    r#"
    SELECT 
      p.paper_name,
      p.paper_src,
      DATE_FORMAT(p.pub_year, '%Y') as pub_year,
      p.paper_type,
      p.paper_level,
      pp.ranking,
      pp.comm_auth
    FROM Publication p
    INNER JOIN PaperPub pp ON p.paper_id = pp.paper_id
    WHERE pp.teacher_id = ?
      AND YEAR(p.pub_year) >= ?
      AND YEAR(p.pub_year) <= ?
    "#,
    &tid, start_year, end_year
  )
  .fetch_all(&pool)
  .await
  .map_db_err()?;

  
  let mut projects = sqlx::query_as!(
    models::ProjectR,
    r#"
    SELECT 
      p.project_id,
      p.project_name,
      p.project_src,
      p.project_type,
      p.start_year,
      p.end_year,
      p.secret_level,
      pr.ranking,
      pr.fund
    FROM Project p
    INNER JOIN ProjectResp pr ON p.project_id = pr.project_id
    WHERE pr.teacher_id = ?
      AND p.start_year <= ?
      AND COALESCE(p.end_year, 100000) >= ?
    "#,
    &tid, start_year, end_year
  )
  .fetch_all(&pool)
  .await
  .map_db_err()?;

  let courses = sqlx::query_as!(
    models::CourseR,
    r#"
    SELECT 
      c.course_name,
      c.course_property,
      ct.course_year,
      ct.course_semester,
      ct.resp_hour
    FROM Course c
    INNER JOIN ClassTeach ct ON c.course_id = ct.course_id
    WHERE ct.teacher_id = ?
      AND ct.course_year >= ?
      AND ct.course_year <= ?
    "#,
    &tid, start_year, end_year
  )
  .fetch_all(&pool)
  .await
  .map_db_err()?;

  // 写入： 在project 检查权限 只有admin和 参与者看得到
  
  let mut write_buffer = r#"#import "lib.typ": *"#.to_string();

  let sy = match query.start_year {
    Some(a) => a.to_string(),
    None => "none".to_string(), 
  };
  let ey = match query.end_year {
    Some(a) => a.to_string(),
    None => "none".to_string(), // 这只是一个充分大的数
  };

  write!(write_buffer, "\n\n#show: resume_template( my_title(\"{}\", {}, {}) )\n",
    teacher.teacher_name, sy, ey
  ).unwrap();

  write!(write_buffer, "== 基本信息\n\n#my_info(id: {}, sex: {}, title: {})\n\n",
    i32::from_str_radix(&teacher.teacher_id, 16).unwrap(), teacher.teacher_sex, teacher.teacher_title
  ).unwrap();
  
  if papers.len() != 0 {
    write!(write_buffer, "== 科研\n\n").unwrap();
  }

  for paper in papers {
    let comm = if paper.comm_auth == 0 { "false".to_string() } else { "true".to_string() };
    write!(write_buffer, "#my_paper(name: \"{}\", year: {}, src: \"{}\", rank: {}, comm: {}, lvl: {}, ty: {} )\n\n",
      paper.paper_name, paper.pub_year.unwrap(), paper.paper_src, paper.ranking, comm, paper.paper_level, paper.paper_type
    ).unwrap();
  }

  // 权限检查，并删除无权限的

  let mut i = 0;
  while i < projects.len() {
    if projects[i].secret_level == 2 && pri == 1 {
      if not_teacher {
        projects.remove(i);
        continue;
      }

      let cnt = sqlx::query_as!(
        models::Cnt,
        r#"
        SELECT COUNT(1) as count
        FROM ProjectResp
        WHERE teacher_id = ?
        AND project_id = ?
        "#,
        &utid, projects[i].project_id
      )
      .fetch_one(&pool).await
      .map_db_err()?;

      if cnt.count == 0 {
        projects.remove(i);
        continue;
      }
    }
    i += 1;
  }

  if projects.len() != 0 {
    write!(write_buffer, "== 项目\n\n").unwrap();
  }

  for project in projects {
    let ey = match project.end_year {
      Some(i) => i.to_string(),
      None => "none".to_string()
    };
    write!(write_buffer, "#my_project(name: \"{}\", start: {}, end: {}, rank: {}, fund: {}, src: \"{}\", secret_lvl: {}, ty: {})\n\n",
      project.project_name, project.start_year, ey, project.ranking, project.fund, project.project_src, project.secret_level, project.project_type
    ).unwrap();
  }

  if courses.len() != 0 {
    write!(write_buffer, "== 课程\n\n").unwrap();
  }

  for course in courses {
    write!(write_buffer, "#my_course(name: \"{}\", hour: {}, year: {}, semester: {})\n\n",
      course.course_name, course.resp_hour, course.course_year, course.course_semester
    ).unwrap();
  }

  let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let filepath = base.join("pdf/main.typ");
  
  // println!("{:?}", filepath);
  let result = tokio::fs::write(filepath, write_buffer).await;
  // println!("{}",write_buffer);

  match result {
    Ok(_) => {
      use std::process::Command;
      let output = Command::new("typst")
        .arg("compile")
        .arg("pdf/main.typ")
        .arg("pdf/out.pdf")
        .output()
        .expect("failed to execute typst compile");

      if !output.status.success() {
        eprintln!("typst compile error: {}", String::from_utf8_lossy(&output.stderr));
        return ret_format("PDF generation failed", StatusCode::INTERNAL_SERVER_ERROR); 
      }

      let pdf_bytes = match fs::read("pdf/out.pdf").await {
        Ok(bytes) => bytes,
        Err(e) => {
          eprintln!("Failed to read PDF: {}", e);
          return ret_format("Failed to read PDF.", StatusCode::INTERNAL_SERVER_ERROR);
        }
      };

      let resp = warp::http::Response::builder()
        .header(warp::http::header::CONTENT_TYPE, "application/pdf")
        .header(warp::http::header::CONTENT_DISPOSITION, "attachment; filename=\"range_query.pdf\"")
        .body(pdf_bytes).unwrap();

      Ok(Box::new(resp))
    },
    Err(e) => {
      ret_format(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    },
  }
    
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
  .execute(&pool).await
  .map_db_err();

  match result {
    Ok(_) => Ok(warp::reply::with_status("Changed", StatusCode::OK)),
    Err(e) => {
      eprintln!("Insert error: {:?}", e);
      Ok(warp::reply::with_status("Change Password Failed.", StatusCode::INTERNAL_SERVER_ERROR))
    }
  }
}