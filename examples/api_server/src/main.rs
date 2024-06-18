use anyhow::Result;
use serde::Deserialize;
use shika::{extract::Extract, Request, serve};
use shika::extract::Query;
use shika_auth::authorize;
use shika_auth::rbac::{Client, Permission};

#[tokio::main]
async fn main() -> Result<()> {
    serve("0.0.0.0:8081", handler).await.unwrap();

    Ok(())
}

#[derive(Deserialize)]
struct QueryParams {
    pub permission: String
}

async fn handler(req: Request) -> Result<String> {
    let query: QueryParams = Query::extract(&req)?;

    let client = Client {
        permissions: vec![Permission::Read(query.permission)]
    };

    authorize!(client, [Permission::Read("users".to_string())]);

    Ok("Hello, Worldie!".to_string())
}