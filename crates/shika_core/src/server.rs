use std::{sync::Arc, future::Future, process::exit};
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tracing::{error, info};

use crate::handler::HandlerService;

pub async fn serve<F, Fut>(address: &str, handler: F)
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future + Send + 'static
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
        let (stream, address) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);

        let service = HandlerService::new(Arc::clone(&handler));

        tokio::task::spawn(async move {
            if let Err(error) = http1::Builder::new()
                .serve_connection(io, service)
            .await {
                error!("Error: {error}")
            }
        });
    }
}
