use std::sync::Arc;

use axum::{
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};
use axum_macros::debug_handler;
use back::UserDeck;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use tokio::sync::mpsc;

use crate::{AppState, server::initialize::create_pvp_game};

pub struct MatchmakingPlayer {
    user_id: Uuid,
    deck: UserDeck,
    tx: mpsc::Sender<MatchmakingMessage>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum MatchmakingMessage {
    Waiting,
    GameFound {
        #[serde(rename = "gameId")]
        game_id: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
enum MatchmakingClientMessage {
    JoinQueue { deck: UserDeck },
}

#[debug_handler]
pub async fn handle_matchmaking(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Response {
    tracing::info!(
        "WebSocket connection request from user to start matchmaking {}",
        user_id
    );

    ws.on_upgrade(move |socket| handle_matchmaking_socket(socket, state, user_id))
}

async fn handle_matchmaking_socket(socket: WebSocket, state: Arc<AppState>, user_id: Uuid) {
    tracing::info!("WebSocket upgraded for user {} in matchmaking", user_id);

    if let Some(existing_game_id) = state.current_live_games.get(&user_id) {
        let game_id = *existing_game_id.value();
        tracing::info!("User {} already has an active game: {}", user_id, game_id);

        let message = MatchmakingMessage::GameFound {
            game_id: game_id.to_string(),
        };
        let serialized = serde_json::to_string(&message).unwrap();

        let (mut ws_sender, _ws_receiver) = socket.split();
        let _ = ws_sender.send(Message::Text(serialized.into())).await;
        let _ = ws_sender.close().await;

        tracing::info!(
            "Sent existing game info to user {} and closed connection",
            user_id
        );
        return;
    }

    let (mut ws_sender, mut ws_receiver) = socket.split();

    let (tx, mut rx) = mpsc::channel::<MatchmakingMessage>(10);

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let serialized = serde_json::to_string(&msg).unwrap();
            if ws_sender
                .send(Message::Text(serialized.into()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    let state_clone = state.clone();
    let tx_clone = tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if let Message::Text(text) = msg {
                tracing::info!("Matchmaking message from {}: {}", user_id, text);

                let client_msg: MatchmakingClientMessage = match serde_json::from_str(&text) {
                    Ok(m) => m,
                    Err(e) => {
                        tracing::error!("Invalid matchmaking message: {}", e);
                        continue;
                    }
                };

                let MatchmakingClientMessage::JoinQueue { deck } = client_msg;

                let opponent = {
                    let mut queue = state_clone.matchmaking_queue.lock().unwrap();
                    if queue.is_empty() {
                        queue.push(MatchmakingPlayer {
                            user_id,
                            deck: deck.clone(),
                            tx: tx_clone.clone(),
                        });
                        None
                    } else {
                        Some(queue.remove(0))
                    }
                };

                if let Some(opponent) = opponent {
                    tracing::info!("Match found! {} vs {}", user_id, opponent.user_id);

                    let game_id = create_pvp_game(
                        &state_clone,
                        user_id,
                        deck,
                        opponent.user_id,
                        opponent.deck,
                    )
                    .await;

                    let _ = opponent
                        .tx
                        .send(MatchmakingMessage::GameFound {
                            game_id: game_id.to_string(),
                        })
                        .await;

                    let _ = tx_clone
                        .send(MatchmakingMessage::GameFound {
                            game_id: game_id.to_string(),
                        })
                        .await;

                    break;
                } else {
                    let _ = tx_clone.send(MatchmakingMessage::Waiting).await;
                }
            }
        }

        state_clone
            .matchmaking_queue
            .lock()
            .unwrap()
            .retain(|p| p.user_id != user_id);

        tracing::info!("User {} left matchmaking", user_id);
    });

    tokio::select! {
        _ = (&mut send_task) => {
            drop(tx);
            let _ = send_task.await;
        },
        _ = (&mut recv_task) => {
            drop(tx);
            let _ = send_task.await;
        },
    }

    tracing::info!("Matchmaking socket closed for user {}", user_id);
}
