mod error;
mod web;
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
    .without_time()
    .with_target(false)
    .with_env_filter(EnvFilter::from_default_env())
    .init();

    info!("{:<12} - Starting Web Server", "STARTUP");

    // -- Define Routes
    let routes_rpc = routes_rpc::routes()
    
    let routes = Router::new();

    let listener = TcpListener::bind("127.0.0.1:8080").await.map_err(|_| Error::TcpListener)?;
    info!("{:<12} -  Listening on: {:?}", "STARTUP", listener.local_addr());
    
    axum::serve(listener, routes.into_make_service())
        .await.map_err(|_| Error::Axum)?;

    Ok(())
}
