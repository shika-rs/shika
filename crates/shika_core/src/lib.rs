use std::collections::HashMap;
use std::future::Future;
use crate::exchange::{Method, Request};

pub mod exchange;
pub mod server;
mod handler;

pub async fn handle<F, S>(handler: F) -> ()
where
    F: Fn(Request) -> S,
    S: Future {
    println!("Before");
    handler(Request {
        path: "/".to_string(),
        headers: HashMap::new(),
        method: Method::Get
    }).await;
    println!("After");
}
