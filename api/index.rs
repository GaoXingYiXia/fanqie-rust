use vercel_runtime::{run, Body, Error, Request, Response};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "rust": "fanqie",
                "message": "参数错误",
                "data": { "content": "没有数据" }
            })
            .to_string()
            .into(),
        )?;
    
    Ok(response)
}