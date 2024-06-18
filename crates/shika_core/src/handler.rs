use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use http::Response;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::service::Service;
use crate::exchange::{Request, RequestBody};

pub struct HandlerService<Handler, ResponseFuture>
    where
        Handler: Fn(Request) -> ResponseFuture + Send + Sync + 'static,
        ResponseFuture: Future<Output = anyhow::Result<String>> + Send + 'static
{
    handler: Arc<Handler>,
}

impl<Handler, ResponseFuture> HandlerService<Handler, ResponseFuture>
    where
        Handler: Fn(Request) -> ResponseFuture + Send + Sync + 'static,
        ResponseFuture: Future<Output = anyhow::Result<String>> + Send + 'static
{
    pub fn new(handler: Arc<Handler>) -> Self {
        HandlerService {
            handler
        }
    }
}

impl<Handler, HandlerFuture> Service<http::Request<RequestBody>> for HandlerService<Handler, HandlerFuture>
    where
        Handler: Fn(Request) -> HandlerFuture + Send + Sync + 'static,
        HandlerFuture: Future<Output = anyhow::Result<String>> + Send + 'static
{
    type Response = http::Response<Full<Bytes>>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, request: http::Request<RequestBody>) -> Self::Future {
        let (parts, body) = request.into_parts();
        let handler = Arc::clone(&self.handler);
        let request = Request {
            parts,
            body
        };

        Box::pin(async move {
            if let Ok(result) = handler(request).await {
                Ok(Response::new(Full::new(Bytes::from(result))))
            } else {
                Ok(Response::new(Full::new(Bytes::from("Error"))))
            }
        })
    }
}
