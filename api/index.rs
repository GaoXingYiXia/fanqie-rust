use vercel_runtime::{run, Error, Request, Response};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<String>, Error> {
    Response::json(&json!({
        "rust": "fanqie",
        "message": "参数错误",
        "data": {
            "content": "没有数据"
        }
    }))
}