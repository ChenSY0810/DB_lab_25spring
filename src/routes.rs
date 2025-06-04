use warp::{filters::path::FullPath, Filter, Rejection};
use sqlx::MySqlPool;
use crate::handlers;
use std::convert::Infallible;

pub fn api_routes(pool: MySqlPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  
  let db = warp::any().map(move || pool.clone());

  let teacher_insert = warp::path!("api" / "teachers")
    .and(warp::post())
    // .and(warp::header::<String>("Authorization"))
    .and(warp::body::json())
    .and(db)
    .and_then(handlers::insert_teacher_handler);

    // debug
    // .recover(|err: Rejection| async move {
    //     if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
    //         eprintln!("Body deserialize error: {:?}", e);
    //     }
    //     Ok::<_, Rejection>(warp::reply::with_status(
    //         "Bad Request",
    //         warp::http::StatusCode::BAD_REQUEST,
    //     ))
    // });


  let total_routes = teacher_insert;
    // .with(warp::cors().allow_any_origin())
    // .recover(handle_rejection)
    
  total_routes
}
