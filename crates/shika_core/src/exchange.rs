#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete
}

pub type RequestParts = http::request::Parts;
pub type RequestBody = hyper::body::Incoming;

pub struct Request {
    pub parts: RequestParts,
    pub body: RequestBody
}
