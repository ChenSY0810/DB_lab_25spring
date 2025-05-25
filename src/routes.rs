use warp::Filter;
use crate::handlers;

pub fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(
            warp::path("hello")
                .and(warp::get())
                .and_then(handlers::hello_handler)
        )
}
