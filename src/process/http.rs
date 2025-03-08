use anyhow::Result;
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Debug)]
pub struct AppState {
    path: PathBuf,
}

async fn index_handler(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> (StatusCode, Response) {
    let path = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", path);
    if !path.exists() {
        (StatusCode::NOT_FOUND, "File not found".into_response())
    } else {
        let content = tokio::fs::read_to_string(path).await;
        match content {
            Ok(content) => (StatusCode::OK, content.into_response()),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", e).into_response(),
            ),
        }
    }
}

pub async fn process_http(path: PathBuf, port: u16) -> Result<()> {
    info!(
        "Processing HTTP request for path: {} on port: {}",
        path.display(),
        port
    );
    let app_state = AppState { path: path.clone() };
    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();

    let router = Router::new()
        .route("/{*lang}", get(index_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(app_state));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service()).await?;
    anyhow::Ok(())
}
