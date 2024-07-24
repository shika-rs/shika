use crate::Request;

mod extension;
mod query;
mod headers;

pub use extension::Extension;
pub use query::Query;
pub use headers::Headers;

pub trait Extract<T> {
    fn extract(request: &Request) -> Result<T, String>;
}