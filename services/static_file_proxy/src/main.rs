use anyhow::Result;
use axum::response::IntoResponse;
use axum::{http::StatusCode, routing::any, Router};
use std::{env, net::SocketAddr};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppConfig {
    static_root: String,
    proxy_mode: bool,
    proxy_target: String,
    base_path: String,
}

async fn proxy_handler() -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "proxy mode not enabled")
}

#[tokio::main]
async fn main() -> Result<()> {
    let port = env::var("PROXY_PORT").unwrap_or_else(|_| "8080".into());
    let static_root = env::var("STATIC_ROOT").unwrap_or_else(|_| "/app/public".into());
    let proxy_mode = env::var("PROXY_MODE").unwrap_or_else(|_| "off".into()) == "on";
    let proxy_target = env::var("PROXY_TARGET").unwrap_or_else(|_| "http://api:8000".into());
    let base_path = env::var("PROXY_BASE_PATH").unwrap_or_else(|_| "/api/".into());

    let cfg = AppConfig {
        static_root: static_root.clone(),
        proxy_mode,
        proxy_target,
        base_path: base_path.clone(),
    };

    let mut app = Router::new().nest_service("/", ServeDir::new(static_root));
    if proxy_mode {
        app = app.fallback(any(proxy_handler));
    }
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.with_state(cfg)).await?;
    Ok(())
}
