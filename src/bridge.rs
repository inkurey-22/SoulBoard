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

/// Handle for publishing score/state updates to connected clients.
pub struct BridgeHandle {
    tx: broadcast::Sender<String>,
    latest_state: Arc<Mutex<TeamState>>,
}
/// Serialize a `TeamState` for publishing over the bridge. Adds top-level
/// `map` and `mode` keys derived from the selected slot (or the first
/// non-empty slot) and removes internal slot vectors from the payload.
pub fn scores_payload(state: &TeamState) -> String {
    match serde_json::to_value(state) {
        Ok(mut value) => {
            if let serde_json::Value::Object(ref mut map) = value {
                map.remove("map_mode_slots");
                map.remove("selected_slot");

                let mut current_map: Option<String> = None;
                let mut current_mode: Option<String> = None;

                if let Some(idx) = state.selected_slot
                    && idx < state.map_mode_slots.len()
                {
                    let (ref map_opt, ref mode_opt) = state.map_mode_slots[idx];
                    if let Some(m) = map_opt
                        && !m.is_empty()
                    {
                        current_map = Some(m.clone());
                    }
                    if let Some(mo) = mode_opt
                        && !mo.is_empty()
                    {
                        current_mode = Some(mo.clone());
                    }
                }

                if current_map.is_none() || current_mode.is_none() {
                    for slot in &state.map_mode_slots {
                        if current_map.is_none()
                            && let Some(ref m) = slot.0
                            && !m.is_empty()
                        {
                            current_map = Some(m.clone());
                        }
                        if current_mode.is_none()
                            && let Some(ref mo) = slot.1
                            && !mo.is_empty()
                        {
                            current_mode = Some(mo.clone());
                        }
                        if current_map.is_some() && current_mode.is_some() {
                            break;
                        }
                    }
                }

                if let Some(m) = current_map {
                    map.insert("map".to_string(), serde_json::Value::String(m));
                }
                if let Some(mo) = current_mode {
                    map.insert("mode".to_string(), serde_json::Value::String(mo));
                }
            }

            serde_json::to_string(&value).unwrap_or_else(|_| {
                serde_json::to_string(&TeamState::default()).unwrap_or_else(|_| "{}".to_string())
            })
        }
        Err(_) => serde_json::to_string(&TeamState::default()).unwrap_or_else(|_| "{}".to_string()),
    }
}

/// Start the HTTP/WebSocket bridge server in a background thread.
///
/// Returns a `BridgeHandle` that can be used to publish state updates
/// to connected clients.
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
    /// Publish a state update to connected clients.
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
