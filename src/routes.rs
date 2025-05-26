use warp::Filter;
use crate::handlers;

pub fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let query = warp::path!("api" / "query")
        .and(warp::get())
        .and(warp::query::<handlers::QueryParams>())
        .and_then(handlers::handle_query);

    let update = warp::path!("api" / "update")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::handle_update);

    let insert = warp::path!("api" / "insert")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::handle_insert);

    let range = warp::path!("api" / "range")
        .and(warp::get())
        .and(warp::query::<handlers::RangeParams>())
        .and_then(handlers::handle_range);

    query.or(update).or(insert).or(range)
}
