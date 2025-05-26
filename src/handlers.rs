use serde::{Deserialize, Serialize};
use warp::Reply;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub field: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    pub id: String,
    pub field: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct InsertRequest {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct RangeParams {
    pub field: String,
    pub start: String,
    pub end: String,
}

// 示例：查询接口
pub async fn handle_query(params: QueryParams) -> Result<impl warp::Reply, warp::Rejection> {
    let msg = format!("查询字段 {} 的值为 {}", params.field, params.value);
    Ok(warp::reply::json(&msg))
}

// 示例：更改接口
pub async fn handle_update(req: UpdateRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let msg = format!("将 ID {} 的 {} 更改为 {}", req.id, req.field, req.value);
    Ok(warp::reply::json(&msg))
}

// 示例：插入接口
pub async fn handle_insert(req: InsertRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let msg = format!("插入新数据：名字={}, 状态={}", req.name, req.status);
    Ok(warp::reply::json(&msg))
}

// 示例：范围查询接口
pub async fn handle_range(params: RangeParams) -> Result<impl warp::Reply, warp::Rejection> {
    let msg = format!(
        "查询字段 {} 从 {} 到 {} 的范围",
        params.field, params.start, params.end
    );
    Ok(warp::reply::json(&msg))
}
