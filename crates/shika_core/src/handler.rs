use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use http_body_util::Full;
use hyper::body::{Body, Bytes};
use hyper::{Request, Response};
use hyper::service::Service;

pub struct HandlerService<F, Fut>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future + Send + 'static
{
    handler: Arc<F>,
}

impl<F, Fut> HandlerService<F, Fut>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future + Send + 'static
{
    pub fn new(handler: Arc<F>) -> Self {
        HandlerService {
            handler
        }
    }
}

impl<F, Fut, O> Service<Request<O>> for HandlerService<F, Fut>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future + Send + 'static,
        O: Body
{
    type Response = Response<Full<Bytes>>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, _req: Request<O>) -> Self::Future {
        let handler = Arc::clone(&self.handler);
        let fut = handler();

        Box::pin(async move {
            fut.await;
            Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
        })
    }
}
