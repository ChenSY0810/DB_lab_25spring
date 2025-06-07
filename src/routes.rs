use warp::Filter;
use sqlx::MySqlPool;
use crate::handlers;

use crate::models;

fn with_db(pool: MySqlPool) -> impl Filter<Extract = (MySqlPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn api_routes(pool: MySqlPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  
  let insert_teacher = warp::path!("api" / "teachers")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(handlers::insert_teacher_handler);

  let list_teacher = warp::path!("api" / "teachers")
    .and(warp::get())
    .and(with_db(pool.clone()))
    .and_then(handlers::list_teacher_handler);
  
  let list_user = warp::path!("api" / "users")
    .and(warp::get())
    .and(with_db(pool.clone()))
    .and_then(handlers::list_user_handler);

  let register = warp::path!("api" / "register")
  .and(warp::post())
  .and(warp::body::json())
  .and(with_db(pool.clone()))
  .and_then(handlers::register_handler);

  let login = warp::path!("api" / "login")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(handlers::login_handler);

  let get_user_id = warp::path!("api" / "users")
    .and(warp::get())
    .and(warp::query::<models::UserName>())
    .and(with_db(pool.clone()))
    .and_then(handlers::get_user_id_handler);

  let link = warp::path!("api"/ "link")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(handlers::link_handler);

  let insert_project = warp::path!("api" / "projects")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(handlers::project_create_handler);

  let change_password = warp::path!("api" / "users")
    .and(warp::post())
    .and(warp::query::<models::UserName>())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(handlers::change_password_handler);

  let total_routes = insert_teacher
    .or(list_teacher)
    .or(list_user)
    .or(register)
    .or(login)
    .or(get_user_id)
    .or(link)
    .or(insert_project)
    .or(change_password);
    // .with(warp::cors().allow_any_origin())
    // .recover(handle_rejection)
    
  total_routes
}
