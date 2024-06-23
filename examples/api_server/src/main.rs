use serde::Deserialize;
use shika::extract::{Query, Extract};
use shika::prelude::*;
use shika::Server;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    Server::bind("0.0.0.0:8081", handler)
        .start()
        .await?;

    Ok(())
}

#[derive(Deserialize)]
struct HandlerQuery {
    name: String,
    age: usize,
    country: Option<String>
}
async fn handler(req: Request) -> Result<String> {
    let HandlerQuery { name, age, country } = Query::extract(&req)?;

    let country = match country {
        Some(country) => country,
        None => "Unknown".to_string()
    };

    Ok(format!("Hello, {name}! You are {age} years old. Country: {country}"))
}
