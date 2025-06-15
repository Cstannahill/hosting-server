use std::{env, net::SocketAddr, sync::Arc};
use axum::{Router, routing::{get, post}, extract::{ws::{WebSocketUpgrade, WebSocket, Message}, State}, response::IntoResponse, Json};
use tokio::sync::broadcast::{self, Sender};
use serde::Deserialize;
use anyhow::Result;

#[derive(Clone)]
struct AppState {
    tx: Sender<String>,
}

#[derive(Deserialize)]
struct BroadcastPayload { message: String }

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| client_connection(socket, state))
}

async fn client_connection(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.tx.subscribe();
    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Ok(text) => if socket.send(Message::Text(text)).await.is_err() { break; },
                    Err(_) => break,
                }
            },
            result = socket.recv() => {
                if let Some(Ok(_)) = result { continue; } else { break; }
            }
        }
    }
}

async fn broadcast(State(state): State<Arc<AppState>>, Json(payload): Json<BroadcastPayload>) -> impl IntoResponse {
    let _ = state.tx.send(payload.message);
    "ok"
}

#[tokio::main]
async fn main() -> Result<()> {
    let port: u16 = env::var("WS_PORT").unwrap_or_else(|_| "9400".into()).parse()?;
    let (tx, _) = broadcast::channel(100);
    let state = Arc::new(AppState { tx });
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/broadcast", post(broadcast))
        .with_state(state);
    let addr = SocketAddr::from(([0,0,0,0], port));
    println!("ws_broadcast listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
