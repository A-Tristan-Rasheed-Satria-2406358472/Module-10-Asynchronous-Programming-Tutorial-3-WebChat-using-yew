use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, RwLock},
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Debug, Serialize)]
struct OutgoingMessageData {
    from: String,
    message: String,
    time: u128,
}

#[derive(Default)]
struct AppState {
    users: HashMap<usize, String>,
    clients: HashMap<usize, mpsc::UnboundedSender<String>>,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind websocket server on 127.0.0.1:8080");

    println!("Listening on port 8080");

    let state = Arc::new(RwLock::new(AppState::default()));
    let mut next_client_id: usize = 1;

    loop {
        let (stream, _) = listener.accept().await.expect("accept failed");
        let client_id = next_client_id;
        next_client_id += 1;

        let state = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(err) = handle_connection(stream, client_id, state).await {
                eprintln!("connection {client_id} error: {err}");
            }
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    client_id: usize,
    state: Arc<RwLock<AppState>>,
) -> Result<(), String> {
    let ws_stream = accept_async(stream)
        .await
        .map_err(|e| format!("websocket handshake failed: {e}"))?;
    println!("ws connected");

    let (mut ws_write, mut ws_read) = ws_stream.split();
    let (out_tx, mut out_rx) = mpsc::unbounded_channel::<String>();

    {
        let mut s = state.write().await;
        s.clients.insert(client_id, out_tx);
    }

    let write_task = tokio::spawn(async move {
        while let Some(payload) = out_rx.recv().await {
            if ws_write.send(Message::Text(payload)).await.is_err() {
                break;
            }
        }
    });

    while let Some(incoming) = ws_read.next().await {
        match incoming {
            Ok(Message::Text(text)) => {
                if let Ok(msg) = serde_json::from_str::<WebSocketMessage>(&text) {
                    match msg.message_type {
                        MsgTypes::Register => {
                            if let Some(username) = msg.data {
                                {
                                    let mut s = state.write().await;
                                    s.users.insert(client_id, username);
                                }
                                broadcast_users(&state).await;
                            }
                        }
                        MsgTypes::Message => {
                            let sender = {
                                let s = state.read().await;
                                s.users
                                    .get(&client_id)
                                    .cloned()
                                    .unwrap_or_else(|| "anonymous".to_string())
                            };

                            let payload_data = OutgoingMessageData {
                                from: sender,
                                message: msg.data.unwrap_or_default(),
                                time: now_ms(),
                            };

                            let payload = WebSocketMessage {
                                message_type: MsgTypes::Message,
                                data_array: None,
                                data: Some(
                                    serde_json::to_string(&payload_data)
                                        .map_err(|e| format!("serialize message data failed: {e}"))?,
                                ),
                            };

                            broadcast_json(&state, &payload).await;
                        }
                        MsgTypes::Users => {}
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Ok(Message::Ping(_)) | Ok(Message::Pong(_)) | Ok(Message::Binary(_)) => {}
            Ok(Message::Frame(_)) => {}
            Err(_) => break,
        }
    }

    {
        let mut s = state.write().await;
        s.clients.remove(&client_id);
        s.users.remove(&client_id);
    }
    println!("ws disconnected");

    broadcast_users(&state).await;
    write_task.abort();

    Ok(())
}

async fn broadcast_users(state: &Arc<RwLock<AppState>>) {
    let users = {
        let s = state.read().await;
        s.users.values().cloned().collect::<Vec<_>>()
    };

    let msg = WebSocketMessage {
        message_type: MsgTypes::Users,
        data_array: Some(users),
        data: None,
    };

    broadcast_json(state, &msg).await;
}

async fn broadcast_json(state: &Arc<RwLock<AppState>>, msg: &WebSocketMessage) {
    if let Ok(serialized) = serde_json::to_string(msg) {
        let clients = {
            let s = state.read().await;
            s.clients.values().cloned().collect::<Vec<_>>()
        };

        for tx in clients {
            let _ = tx.send(serialized.clone());
        }
    }
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}
