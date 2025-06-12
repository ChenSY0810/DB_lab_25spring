use warp::Filter;
use sqlx::MySqlPool;
use crate::handlers::*;

use crate::models;

fn with_db(pool: MySqlPool) -> impl Filter<Extract = (MySqlPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn api_routes(pool: MySqlPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  
  let insert_teacher = warp::path!("api" / "teachers")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(insert_teacher_handler);

  let list_teacher = warp::path!("api" / "teachers")
    .and(warp::get())
    .and(with_db(pool.clone()))
    .and_then(list_teacher_handler);
  
  let list_user = warp::path!("api" / "users")
    .and(warp::get())
    .and(with_db(pool.clone()))
    .and_then(list_user_handler);

  let register = warp::path!("api" / "register")
  .and(warp::post())
  .and(warp::body::json())
  .and(with_db(pool.clone()))
  .and_then(register_handler);

  let login = warp::path!("api" / "login")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(login_handler);

  let get_user_id = warp::path!("api" / "users")
    .and(warp::get())
    .and(warp::query::<models::UserName>())
    .and(with_db(pool.clone()))
    .and_then(get_user_id_handler);

  let link = warp::path!("api"/ "link")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(link_handler);

  let insert_project = warp::path!("api" / "projects")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(project_create_handler);

  let query_project = warp::path!("api" / "projects" / "query")
    .and(warp::post())
    .and(warp::query::<models::UserName>())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(project_read_handler);

  let update_project = warp::path!("api" / "projects" / "update")
    .and(warp::put())
    .and(warp::query::<models::UserName>())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(project_update_handler);

  let delete_project = warp::path!("api" / "projects")
    .and(warp::delete())
    .and(warp::query::<models::UserName>())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(project_delete_handler);

  let insert_paper = warp::path!("api" / "papers")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(paper_create_handler);

  let query_paper = warp::path!("api" / "papers" / "query")
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(paper_read_handler);

  let update_paper = warp::path!("api" / "papers" / "update")
    .and(warp::put())
    .and(warp::query::<models::UserName>())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(paper_update_handler);

  let delete_paper = warp::path!("api" / "papers")
    .and(warp::delete())
    .and(warp::query::<models::UserName>())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(paper_delete_handler);

  let query_course = warp::path!("api" / "courses" / "query")
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(course_read_handler);

  let insert_course = warp::path!("api" / "courses")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(course_create_handler);

  let update_course = warp::path!("api" / "courses" / "update")
    .and(warp::put())
    .and(warp::query::<models::UserName>())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(course_update_handler);

  let delete_course = warp::path!("api" / "courses")
    .and(warp::delete())
    .and(warp::query::<models::UserName>())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(course_delete_handler);

  let change_password = warp::path!("api" / "users")
    .and(warp::post())
    .and(warp::query::<models::UserName>())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(change_password_handler);

  let total_routes = insert_teacher
    .or(list_teacher)
    .or(list_user)
    .or(register)
    .or(login)
    .or(get_user_id)
    .or(link)

    .or(insert_project)
    .or(query_project)
    .or(update_project)
    .or(delete_project)

    .or(insert_paper)
    .or(query_paper)
    .or(update_paper)
    .or(delete_paper)

    .or(insert_course)
    .or(query_course)
    .or(update_course)
    .or(delete_course)

    .or(change_password);
    
  total_routes
}
