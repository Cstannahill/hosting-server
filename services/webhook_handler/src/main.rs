use std::{env, process::Stdio};
use axum::{routing::post, Router, extract::State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tokio::{process::Command, sync::Mutex};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    secret: String,
    script: Option<String>,
}

#[axum::debug_handler]
async fn handle_webhook(
    State(state): State<Arc<Mutex<AppState>>>,
    headers: axum::http::HeaderMap,
    body: bytes::Bytes,
) -> impl IntoResponse {
    let signature = headers.get("X-Hub-Signature-256").and_then(|v| v.to_str().ok());
    let secret = { state.lock().await.secret.clone() };
    if let Some(sig) = signature {
        if verify_signature(&secret, &body, sig) {
            if let Some(script) = { state.lock().await.script.clone() } {
                let _ = run_script(&script).await;
            }
            StatusCode::OK
        } else {
            StatusCode::UNAUTHORIZED
        }
    } else {
        StatusCode::BAD_REQUEST
    }
}

fn verify_signature(secret: &str, body: &[u8], signature: &str) -> bool {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("hmac can take key of any size");
    mac.update(body);
    let result = mac.finalize();
    let expected = hex::encode(result.into_bytes());
    let sig_clean = signature.trim_start_matches("sha256=");
    expected.eq_ignore_ascii_case(sig_clean)
}

async fn run_script(path: &str) {
    let _ = Command::new(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();
}

#[tokio::main]
async fn main() {
    let port = env::var("WEBHOOK_PORT").unwrap_or_else(|_| "9000".into());
    let secret = env::var("WEBHOOK_SECRET").unwrap_or_else(|_| "secret".into());
    let script = env::var("ACTION_SCRIPT").ok();
    let state = Arc::new(Mutex::new(AppState { secret, script }));

    let app = Router::new().route("/", post(handle_webhook)).with_state(state);
    let addr = format!("0.0.0.0:{}", port);
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
