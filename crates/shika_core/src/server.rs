use std::{sync::Arc, future::Future, process::exit};
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpListener;
use tracing::{error, info};
use hyper_util::server::conn::auto::Builder;

use crate::exchange::Request;
use crate::handler::HandlerService;

pub struct Server<Handler, HandlerFuture, State = ()>
where
    Handler: Fn(Request) -> HandlerFuture + Send + Sync + 'static,
    HandlerFuture: Future<Output = anyhow::Result<String>> + Send + 'static
{
    address: &'static str,
    handler: Handler,
    state: State
}

impl<Handler, HandlerFuture, State> Server<Handler, HandlerFuture, State>
where
    Handler: Fn(Request) -> HandlerFuture + Send + Copy + Sync + 'static,
    HandlerFuture: Future<Output = anyhow::Result<String>> + Send + 'static,
    State: Clone + Send + Sync + 'static
{
    pub fn bind(address: &'static str, handler: Handler) -> Self {
        Server {
            address,
            handler
        }
    }

    pub async fn start(self) -> anyhow::Result<()> {
        let listener = match TcpListener::bind(self.address).await {
            Ok(listener) => {
                info!("Listening for connections on {}...", self.address);
                listener
            },
            Err(error) => {
                error!("Could not create listener: {error}");
                exit(1);
            }
        };
    
        let handler = Arc::new(self.handler);
    
        loop {
            let (stream, _address) = listener.accept().await.unwrap();
            let io = TokioIo::new(stream);
    
            let service = HandlerService::new(Arc::clone(&handler));
    
            tokio::spawn(async move {
                if let Err(error) = Builder::new(TokioExecutor::new())
                    .serve_connection_with_upgrades(io, service)
                    .await
                {
                    return Err(error)
                }
    
                Ok(())
            });
        }
    }
}
