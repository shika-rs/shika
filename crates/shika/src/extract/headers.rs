use std::collections::HashMap;
use crate::extract::Extract;
use crate::Request;

pub struct Headers;
impl Extract<HashMap<String, String>> for Headers {
    fn extract(request: &Request) -> Result<HashMap<String, String>, String> {
        let mut map = HashMap::new();

        request.headers().iter().for_each(|(name, value)| {
            let key = String::from(name.as_str());
            let value = String::from(value.to_str().unwrap());

            map.insert(key, value);
        });

        Ok(map)
    }
}