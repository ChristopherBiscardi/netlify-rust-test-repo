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
    dbg!("in main", &event.as_str());
    let first_name = event["queryStringParameters"]
        ["firstName"]
        .as_str()
        .unwrap_or("world");

    dbg!(&first_name);
    Ok(json!({
        "body": format!("Hello, {}!", first_name)
    }))
}
