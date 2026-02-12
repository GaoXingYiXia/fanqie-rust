use serde_json::json;
use vercel_runtime::{run, String, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<String>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "rust": "fanqie",
              "message": "参数错误",
              "data": {
                  "content": "没有数据"
              }
            })
            .to_string()
            .into(),
        )?)
}