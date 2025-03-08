use anyhow::{Ok, Result};
use axum::{
    Router,
    response::{IntoResponse, Response},
    routing::get,
};
use std::{net::SocketAddr, path::Path};
use tracing::info;

async fn index_handler() -> Response {
    "Hello, World!".into_response()
}

pub async fn process_http(path: &Path, port: u16) -> Result<()> {
    info!(
        "Processing HTTP request for path: {} on port: {}",
        path.display(),
        port
    );
    let router = Router::new().route("/", get(index_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
