use serde::Deserialize;
use shika::{Request, server};
use shika::extract::{Extension, Extract, Query};

const ADDRESS: &str = "0.0.0.0:8081";

#[derive(Clone)]
struct AppState {
    message: String
}

#[tokio::main]
async fn main() {
    server::bind(ADDRESS, index)
        .extend(AppState {
            message: "Username: ".to_string()
        })
        .listen()
        .await;
}

#[derive(Deserialize)]
struct IndexQuery {
    name: Option<String>
}

async fn index(req: Request) -> Result<String, String> {
    let AppState { message }: AppState = Extension::extract(&req)?;
    let IndexQuery { name }: IndexQuery = Query::extract(&req)?;

    Ok(format!(" {}{}", message, name.unwrap_or_else(|| "Unnamed".to_string())))
}
