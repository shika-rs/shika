use anyhow::anyhow;
use serde::de::DeserializeOwned;
use crate::exchange::Request;
use crate::extract::Extract;

pub struct Query;
impl<T> Extract<T> for Query
where
    T: DeserializeOwned
{
    fn extract(request: &Request) -> anyhow::Result<T> {
        let query = &request.0.uri.query().unwrap_or_default();
        let params = serde_urlencoded::from_str(query);

        match params {
            Ok(params) => Ok(params),
            Err(error) => Err(anyhow!(error.to_string()))
        }
    }
}
