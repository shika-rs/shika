use http_body_util::Full;
use hyper::body::Bytes;

pub type Response = http::Response<Full<Bytes>>;

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response::new(Full::new(Bytes::from(self)))
    }
}

impl<O, E> IntoResponse for Result<O, E>
where
    O: IntoResponse,
    E: IntoResponse
{
    fn into_response(self) -> Response {
        match self {
            Ok(ok) => ok.into_response(),
            Err(err) => err.into_response()
        }
    }
}