use http_body_util::Full;
use hyper::body::Bytes;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete
}

pub type RequestBody = hyper::body::Incoming;

pub type RequestParts = http::request::Parts;
pub type Response<T = Full<Bytes>> = http::Response<T>;

pub struct Request(pub RequestParts, pub RequestBody);