use serde_json::json;
use vercel_runtime::{run, Error, Request, Response, StatusCode};

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
              "message": "Hello from Rust!",
              "status": "success"
            })
            .to_string()
            .into(),
        )?)
}