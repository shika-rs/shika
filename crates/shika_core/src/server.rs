use std::{sync::Arc, future::Future, process::exit};
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpListener;
use tracing::{error, info};
use hyper_util::server::conn::auto::Builder;

use crate::exchange::Request;
use crate::handler::HandlerService;

pub async fn serve<Handler, HandlerFuture>(address: &str, handler: Handler) -> anyhow::Result<()>
where
    Handler: Fn(Request) -> HandlerFuture + Send + Sync + 'static,
    HandlerFuture: Future<Output = anyhow::Result<String>> + Send + 'static
{
    tracing_subscriber::fmt::init();

    let listener = match TcpListener::bind(address).await {
        Ok(listener) => {
            info!("Listening for connections on {address}...");
            listener
        },
        Err(error) => {
            error!("Could not create listener: {error}");
            exit(1);
        }
    };

    let handler = Arc::new(handler);

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
