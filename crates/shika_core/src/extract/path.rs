use crate::exchange::Request;
use crate::extract::Extract;

pub struct Path;
impl<T> Extract<T> for Path {
    fn extract(_request: &Request) -> anyhow::Result<T> {
        todo!();
    }
}