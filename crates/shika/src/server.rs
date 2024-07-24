use std::future::Future;
use std::sync::Arc;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use tokio::net::TcpListener;
use crate::{handler, IntoResponse, Request};

pub fn bind<H>(address: &'static str, handler: H) -> Server<H, ()>
{
    Server {
        address,
        handler,
        extension: None
    }
}

// region: Server
pub struct Server<H, E>
{
    address: &'static str,
    handler: H,
    extension: Option<E>
}

impl<H> Server<H, ()> {
    pub fn extend<E>(self, extension: E) -> Server<H, E> {
        Server {
            address: self.address,
            handler: self.handler,
            extension: Some(extension)
        }
    }
}

impl<H, E> Server<H, E> {
    pub async fn listen<HF, HFR>(self)
    where
        H: Fn(Request) -> HF + Send + Sync + 'static,
        HF: Future<Output = HFR> + Send + Sync + 'static,
        HFR: IntoResponse + Send + Sync + 'static,
        E: Send + Sync + Clone + 'static
    {
        let listener = TcpListener::bind(self.address)
            .await
            .unwrap();

        let handler = Arc::new(self.handler);
        let extension = Arc::new(self.extension);

        loop {
            let (stream, _) = listener.accept().await.unwrap();

            let io = TokioIo::new(stream);

            let handler = handler.clone();
            let extension = extension.clone();

            tokio::spawn(async move {
                Builder::new(TokioExecutor::new())
                    .serve_connection_with_upgrades(io, handler::Service::new(handler, extension))
                    .await
                    .expect("Could not serve connection");
            });
        }
    }
}

// endregion: Server
