use crate::extract::Extract;
use crate::Request;

pub struct Extension;

impl<T> Extract<T> for Extension
where
    T: Send + Sync + Clone + 'static
{
    fn extract(request: &Request) -> Result<T, String> {
        if let Some(extension) = request.extensions().get::<T>() {
            return Ok(extension.clone());
        }

        Err("No extension of the provided type exists".to_string())
    }
}