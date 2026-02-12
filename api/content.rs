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
            Body::Text(  // 明确指定为文本类型
                json!({
                    "message": "Hello from Content!",
                    "status": "success"
                })
                .to_string()
            )
        )?;
    
    Ok(response)
}