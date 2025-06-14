use axum::{Router, routing::get, response::Html};
use axum::extract::State;
use serde::Serialize;
use std::{collections::HashMap, env, time::Duration};
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::{time, task};

#[derive(Clone, Serialize)]
struct Status { ok: bool, response_time_ms: u128 }

type SharedStatus = Arc<Mutex<HashMap<String, Status>>>;

async fn health(State(status): State<SharedStatus>) -> axum::Json<HashMap<String, Status>> {
    axum::Json(status.lock().clone())
}

async fn dashboard(State(status): State<SharedStatus>) -> Html<String> {
    let data = status.lock();
    let mut html = String::from("<html><body><h1>Healthcheck</h1><ul>");
    for (name, st) in data.iter() {
        let color = if st.ok { "green" } else { "red" };
        html.push_str(&format!("<li style='color:{color}'>{name}: {} ms</li>", st.response_time_ms));
    }
    html.push_str("</ul></body></html>");
    Html(html)
}

#[derive(Clone)]
struct Endpoint { name: String, url: String }

async fn check_loop(endpoints: Vec<Endpoint>, interval: Duration, status: SharedStatus) {
    let client = reqwest::Client::new();
    loop {
        for ep in &endpoints {
            let start = time::Instant::now();
            let res = client.get(&ep.url).send().await;
            let ok = res.map(|r| r.status().is_success()).unwrap_or(false);
            let elapsed = start.elapsed().as_millis();
            status.lock().insert(ep.name.clone(), Status { ok, response_time_ms: elapsed });
        }
        time::sleep(interval).await;
    }
}

#[tokio::main]
async fn main() {
    let port = env::var("DASHBOARD_PORT").unwrap_or_else(|_| "9100".into());
    let endpoints = env::var("CHECK_ENDPOINTS").unwrap_or_else(|_| "web:http://web:3000/,api:http://api:8000/health".into());
    let interval = env::var("CHECK_INTERVAL_SECS").ok().and_then(|s| s.parse().ok()).unwrap_or(10);
    let eps: Vec<Endpoint> = endpoints.split(',').filter_map(|s| {
        let mut parts = s.splitn(2, ':');
        Some(Endpoint { name: parts.next()?.to_string(), url: parts.next()?.to_string() })
    }).collect();

    let status: SharedStatus = Arc::new(Mutex::new(HashMap::new()));
    let task_status = status.clone();
    task::spawn(check_loop(eps, Duration::from_secs(interval), task_status));

    let app = Router::new()
        .route("/health", get(health))
        .route("/dashboard", get(dashboard))
        .with_state(status);

    let addr = format!("0.0.0.0:{}", port);
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
