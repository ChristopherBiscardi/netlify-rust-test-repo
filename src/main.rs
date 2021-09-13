use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dbg!("cold start");
    let handler_fn = handler_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(
    event: Value,
    _: Context,
) -> Result<Value, Error> {
    dbg!("in main");
    let first_name =
        event["firstName"].as_str().unwrap_or("world");

    Ok(json!({
        "message": format!("Hello, {}!", first_name)
    }))
}
