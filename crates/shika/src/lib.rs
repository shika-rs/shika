//! # Shika
//! Shika is a web framework for building web applications in Rust.
//! 
//! ## Example
//! ```rust
//! use shika::prelude::*;
//! use shika::server::serve;
//! use shika::extract::{Query, Extract};
//! use serde::Deserialize;
//! 
//! #[derive(Deserialize)]
//! struct IndexQuery {
//!   name: Option<String>
//! }
//! 
//! async fn handler(req: Request) -> Result<String> {
//!   let query: IndexQuery = Query::extract(&req)?;
//! 
//!   match query.name {
//!     Some(name) => Ok(format!("Hello, {}!", name)),
//!     None => Ok("Hello, World!".to_string())
//!   }
//! }
//! ```

pub mod prelude;
pub use shika_core::server::Server;
pub use shika_core::exchange;
pub use shika_core::router;
pub use shika_core::extract;
