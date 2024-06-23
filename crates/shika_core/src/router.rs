use anyhow::anyhow;
use crate::exchange::Request;

pub struct Router;

impl Router {
    pub fn route(&self, _request: &Request) -> anyhow::Result<String> {
        Err(anyhow!("An error occurred"))
    }
}

pub struct Builder {

}

impl Builder {
    pub fn new() -> Self {
        Builder {}
    }

    pub fn build(self) -> Router {
        Router {}
    }
}