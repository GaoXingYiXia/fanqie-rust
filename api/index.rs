use serde_json::json;
use vercel_runtime::{run, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(_req: Request) -> Result<Response<String>, Error> {
    Response::json(&json!({
        "rust": "fanqie",
        "message": "参数错误",
        "data": {
            "content": "没有数据"
        }
    }))
}