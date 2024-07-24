use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::service;
use crate::{IntoResponse, request::Request};

// region: Handler Service
pub struct Service<H, HF, HFR, E>
where
    H: Fn(Request) -> HF + Send + Sync + 'static,
    HF: Future<Output = HFR> + Send + Sync + 'static,
    HFR: IntoResponse + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static
{
    handler: Arc<H>,
    extension: Arc<Option<E>>
}

impl<H, HF, HFR, E> Service<H, HF, HFR, E>
where
    H: Fn(Request) -> HF + Send + Sync + 'static,
    HF: Future<Output = HFR> + Send + Sync + 'static,
    HFR: IntoResponse + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static
{
    pub fn new(handler: Arc<H>, extension: Arc<Option<E>>) -> Self {
        Service {
            handler,
            extension
        }
    }
}

impl<H, HF, HFR, E> service::Service<http::Request<Incoming>> for Service<H, HF, HFR, E>
where
    H: Fn(Request) -> HF + Send + Sync + 'static,
    HF: Future<Output = HFR> + Send + Sync + 'static,
    HFR: IntoResponse + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static
{
    type Response = http::Response<Full<Bytes>>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, mut req: http::Request<Incoming>) -> Self::Future {
        let extension = self.extension.as_ref().clone();

        if let Some(ext) = extension {
            req.extensions_mut().insert(ext);
        }

        let handler_fut = (self.handler)(req);

        Box::pin(async move {
            Ok(handler_fut.await.into_response())
        })
    }
}
// endregion
