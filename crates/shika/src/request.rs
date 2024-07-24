use hyper::body::Incoming;

pub type Request = http::Request<Incoming>;
