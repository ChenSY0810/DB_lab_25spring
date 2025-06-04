mod routes;
mod handlers;
mod db;
mod models;

use warp::Filter;
use warp::http::StatusCode;


#[tokio::main]
async fn main() {
  let pool = db::init_pool().await;
  
  let api_routes = routes::api_routes(pool);
  let static_files = warp::fs::dir("frontend/dist");
  
  let index_html = warp::path::end()
  .and(warp::fs::file("frontend/dist/index.html"));

  let routes = api_routes
    .or(index_html)
    .or(static_files)
    .with(warp::log::custom(|info| {
        println!(
            "{} {} {} {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.request_headers()
        );
    }));

  warp::serve(routes)
    .run(([127, 0, 0, 1], 3030))
    .await;

}
