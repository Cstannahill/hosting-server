use axum::{Router, routing::get, response::IntoResponse, extract::State};
use prometheus::{Encoder, TextEncoder, Registry, IntCounterVec};
use tokio::{time, sync::Mutex};
use std::{env, net::SocketAddr, sync::Arc, time::Duration};
use reqwest::Client;

#[derive(Clone)]
struct AppState {
    registry: Registry,
    requests: IntCounterVec,
    targets: Vec<String>,
    interval: Duration,
}

async fn metrics(State(state): axum::extract::State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let state = state.lock().await;
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let metric_families = state.registry.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

async fn scrape_loop(state: Arc<Mutex<AppState>>) {
    let client = Client::new();
    loop {
        {
            let s = state.lock().await;
            for target in &s.targets {
                if let Ok(resp) = client.get(target).send().await {
                    let label = target.clone();
                    s.requests.with_label_values(&[&label]).inc_by(1);
                    drop(resp);
                }
            }
        }
        time::sleep(state.lock().await.interval).await;
    }
}

#[tokio::main]
async fn main() {
    let port = env::var("EXPORTER_PORT").unwrap_or_else(|_| "9300".into()).parse().unwrap();
    let interval = env::var("SCRAPE_INTERVAL_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(15);
    let targets_env = env::var("SCRAPE_URLS").unwrap_or_default();
    let targets: Vec<String> = targets_env.split(',').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    let registry = Registry::new();
    let requests = IntCounterVec::new(prometheus::Opts::new("scrape_requests_total", "Number of scrapes"), &["target"]).unwrap();
    registry.register(Box::new(requests.clone())).unwrap();

    let state = Arc::new(Mutex::new(AppState {
        registry,
        requests,
        targets,
        interval: Duration::from_secs(interval as u64),
    }));

    let scrape_state = state.clone();
    tokio::spawn(scrape_loop(scrape_state));

    let app = Router::new().route("/metrics", get(metrics)).with_state(state);
    let addr = SocketAddr::from(([0,0,0,0], port));
    println!("metrics exporter listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

