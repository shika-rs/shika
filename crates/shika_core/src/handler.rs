use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::service::Service;
use crate::exchange::{Response, Request, RequestBody};

/// A Hyper Service that wraps a handler function and calls it with a Request.
/// 
pub struct HandlerService<Handler, HandlerFuture>
    where
        Handler: Fn(Request) -> HandlerFuture + Send + Sync + 'static,
        HandlerFuture: Future<Output = anyhow::Result<String>> + Send + 'static
{
    handler: Arc<Handler>
}

impl<Handler, HandlerFuture> HandlerService<Handler, HandlerFuture>
    where
        Handler: Fn(Request) -> HandlerFuture + Send + Sync + 'static,
        HandlerFuture: Future<Output = anyhow::Result<String>> + Send + 'static
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
    type Response = Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, request: http::Request<RequestBody>) -> Self::Future {
        let (parts, body) = request.into_parts();
        let handler = Arc::clone(&self.handler);
        let request = Request(parts, body);

        Box::pin(async move {
            let result = handler(request).await;

            let handler_response = result.unwrap_or_else(|error| error.to_string());

            let response = Response::builder()
                .status(500)
                .body(Full::new(Bytes::from(handler_response)))
                .unwrap();

            Ok(response)
        })
    }
}
