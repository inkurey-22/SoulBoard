use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    Router,
    extract::{
        State,
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::get,
};
use tokio::sync::broadcast;

use crate::model::TeamState;

#[derive(Clone)]
struct BridgeState {
    tx: broadcast::Sender<String>,
    latest_state: Arc<Mutex<TeamState>>,
}

pub struct BridgeHandle {
    tx: broadcast::Sender<String>,
    latest_state: Arc<Mutex<TeamState>>,
}

pub fn scores_payload(state: &TeamState) -> String {
    serde_json::to_string(state).unwrap_or_else(|_| {
        "{\"description\":\"\",\"team_a_name\":\"Team A\",\"team_b_name\":\"Team B\",\"team_a\":0,\"team_b\":0}"
            .to_string()
    })
}

pub fn start_bridge(initial_state: TeamState) -> BridgeHandle {
    let (tx, _rx) = broadcast::channel::<String>(64);
    let latest_state = Arc::new(Mutex::new(initial_state.clone()));
    let _ = tx.send(scores_payload(&initial_state));

    let bridge_state = BridgeState {
        tx: tx.clone(),
        latest_state: Arc::clone(&latest_state),
    };

    std::thread::spawn(move || {
        let runtime = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(err) => {
                eprintln!("Failed to create Tokio runtime: {err}");
                return;
            }
        };

        runtime.block_on(async move {
            let app = Router::new()
                .route("/score", get(http_score))
                .route("/ws", get(ws_upgrade))
                .with_state(bridge_state);

            let addr: SocketAddr = "127.0.0.1:7878".parse().expect("valid socket addr");
            let listener = match tokio::net::TcpListener::bind(addr).await {
                Ok(listener) => listener,
                Err(err) => {
                    eprintln!("Bridge server bind failed on {addr}: {err}");
                    return;
                }
            };

            if let Err(err) = axum::serve(listener, app).await {
                eprintln!("Bridge server error: {err}");
            }
        });
    });

    BridgeHandle { tx, latest_state }
}

impl BridgeHandle {
    pub fn publish_state(&self, state: &TeamState) {
        if let Ok(mut guard) = self.latest_state.lock() {
            *guard = state.clone();
        }

        let _ = self.tx.send(scores_payload(state));
    }
}

async fn http_score(State(state): State<BridgeState>) -> impl IntoResponse {
    let current = match state.latest_state.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => TeamState::default(),
    };

    scores_payload(&current)
}

async fn ws_upgrade(ws: WebSocketUpgrade, State(state): State<BridgeState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: BridgeState) {
    let initial_state = match state.latest_state.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => TeamState::default(),
    };

    if socket
        .send(WsMessage::Text(scores_payload(&initial_state).into()))
        .await
        .is_err()
    {
        return;
    }

    let mut rx = state.tx.subscribe();

    while let Ok(payload) = rx.recv().await {
        if socket.send(WsMessage::Text(payload.into())).await.is_err() {
            break;
        }
    }
}
