use warp::Reply;

pub async fn hello_handler() -> Result<impl Reply, warp::Rejection> {
    Ok(warp::reply::json(&"Hello from backend!"))
}
