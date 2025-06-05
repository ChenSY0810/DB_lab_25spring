use warp::{filters::{body::json, path::FullPath}, Filter, Rejection};
use sqlx::MySqlPool;
use crate::handlers;
use std::convert::Infallible;

use crate::models;

fn with_db(pool: MySqlPool) -> impl Filter<Extract = (MySqlPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn api_routes(pool: MySqlPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  
  let login = warp::path!("api" / "login")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(handlers::login_handler);

  let register = warp::path!("api" / "register")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(handlers::register_handler);

  let insert_teacher = warp::path!("api" / "teachers")
    .and(warp::post())
    .and(warp::header::<String>("Authorization"))
    .and(warp::body::json())
    .and(with_db(pool.clone()))
    .and_then(handlers::insert_teacher_handler);

  let list_teacher = warp::path!("api" / "teachers")
    .and(warp::get())
    .and(warp::header::<String>("Authorization"))
    .and(with_db(pool.clone()))
    .and_then(handlers::list_teacher_handler);

  let get_user_id = warp::path!("api" / "users")
    .and(warp::get())
    .and(warp::query::<models::UserName>())
    .and(with_db(pool.clone()))
    .and_then(handlers::get_user_id_handler);

  let total_routes = login
    .or(register)
    .or(insert_teacher)
    .or(list_teacher)
    .or(get_user_id);
    // .with(warp::cors().allow_any_origin())
    // .recover(handle_rejection)
    
  total_routes
}
