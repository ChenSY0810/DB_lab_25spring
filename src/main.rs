mod routes;
mod handlers;
mod models;

use warp::Filter;

#[tokio::main]
async fn main() {
    // 提供静态前端资源
    let static_files = warp::fs::dir("frontend/dist");

    // 后端 API
    let api = routes::api();

    // 合并路由
    let routes = api.or(static_files);

    println!("Server running at http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
