use vercel_runtime::{run, Body, Error, Request, Response};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let json_string = json!({
        "rust": "fanqie",
        "message": "参数错误",
        "data": { "content": "没有数据" }
    }).to_string();
    
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .header("Cache-Control", "no-cache")  // 添加缓存控制
        .body(Body::from(json_string))?;
    
    Ok(response)
}