use serde::de::DeserializeOwned;
use crate::extract::Extract;
use crate::Request;

pub struct Query;

impl<T> Extract<T> for Query
where
    T: DeserializeOwned
{
    fn extract(request: &Request) -> Result<T, String>{
        let query = &request.uri().query().unwrap_or_default();
        let params = serde_urlencoded::from_str(query);

        match params {
            Ok(params) => Ok(params),
            Err(error) => Err(error.to_string())
        }
    }
}