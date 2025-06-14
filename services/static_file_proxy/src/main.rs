use axum::{Router, routing::get_service, http::{StatusCode, Request}};
use axum::extract::State;
use axum::body;
use std::{env, net::SocketAddr};
use tower_http::services::ServeDir;
use axum::body::Body;
use reqwest::Client;

#[derive(Clone)]
struct AppConfig {
    port: u16,
    static_root: String,
    proxy_mode: bool,
    proxy_target: Option<String>,
}

impl AppConfig {
    fn from_env() -> Self {
        let port = env::var("STATIC_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(8080);
        let static_root = env::var("STATIC_ROOT").unwrap_or_else(|_| "./public".into());
        let proxy_mode = env::var("PROXY_MODE").map(|v| v == "on").unwrap_or(false);
        let proxy_target = env::var("PROXY_TARGET").ok();
        Self { port, static_root, proxy_mode, proxy_target }
    }
}

async fn proxy(State(cfg): State<AppConfig>, req: Request<Body>) -> Result<axum::response::Response, StatusCode> {
    if let Some(target) = &cfg.proxy_target {
        let client = Client::new();
        let path = req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("");
        let url = format!("{}{}", target, path);
        let mut builder = client.request(req.method().clone(), &url);
        builder = builder.headers(req.headers().clone());
        let body_bytes = body::to_bytes(req.into_body(), usize::MAX)
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?;
        let resp = builder.body(body_bytes).send().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
        let mut response = axum::http::Response::builder().status(resp.status());
        for (name, value) in resp.headers() {
            response = response.header(name, value);
        }
        let bytes = resp.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
        Ok(response.body(Body::from(bytes)).unwrap())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {
    let cfg = AppConfig::from_env();

    let serve_dir = get_service(ServeDir::new(&cfg.static_root))
        .handle_error(|_| async { StatusCode::INTERNAL_SERVER_ERROR });

    let mut router = Router::new().nest_service("/", serve_dir);
    if cfg.proxy_mode {
        router = router.fallback(proxy);
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router.with_state(cfg)).await.unwrap();
}


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
