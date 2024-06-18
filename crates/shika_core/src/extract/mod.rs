mod query;
mod path;

use crate::exchange::Request;

pub trait Extract<T> {
    fn extract(request: &Request) -> anyhow::Result<T>;
}

pub use query::Query;
pub use path::Path;


