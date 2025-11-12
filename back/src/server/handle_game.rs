use std::sync::Arc;

use axum::{
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_macros::debug_handler;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    AppState,
    server::initialize::{GameCommand, GameHandle, ServerMessage},
};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum PlayerActionCommand {
    PlayMonster {
        #[serde(rename = "cardId")]
        card_id: usize,
        position: usize,
    },
    PlaySpell {
        #[serde(rename = "cardId")]
        card_id: usize,
    },
    EndTurn,
    Attack {
        initiator: usize,
        target: usize,
    },
    Move {
        #[serde(rename = "cardId")]
        card_id: usize,
        position: usize,
    },
}

#[debug_handler]
pub async fn handle_game(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Path((_game_id, user_id)): Path<(Uuid, Uuid)>,
) -> Response {
    tracing::info!("WebSocket connection request from user {}", user_id);

    let game_id = match state.current_live_games.get(&user_id) {
        Some(entry) => *entry.value(),
        None => {
            tracing::error!("User {} has no active game", user_id);
            return (StatusCode::NOT_FOUND, "No active game").into_response();
        }
    };

    let game_handle = match state.games.get(&game_id) {
        Some(entry) => entry.value().clone(),
        None => {
            tracing::error!("Game {} not found", game_id);
            return (StatusCode::NOT_FOUND, "Game not found").into_response();
        }
    };

    ws.on_upgrade(move |socket| handle_socket(socket, user_id, game_handle))
}

async fn handle_socket(socket: WebSocket, user_id: Uuid, game: GameHandle) {
    tracing::info!("WebSocket upgraded for user {}", user_id);

    let (mut ws_sender, mut ws_receiver) = socket.split();

    let (tx, mut rx) = mpsc::channel::<ServerMessage>(100);

    if game
        .tx
        .send(GameCommand::Connected { user_id, ws_tx: tx })
        .await
        .is_err()
    {
        tracing::error!("Game task is dead, can't connect player {}", user_id);
        return;
    }

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let serialized_action = serde_json::to_string(&msg).unwrap();
            if ws_sender
                .send(Message::Text(serialized_action.into()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    let game_tx = game.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if let Message::Text(text) = msg {
                tracing::info!("Received from user {}: {}", user_id, text);
                let deserialized_action: PlayerActionCommand = serde_json::from_str(&text).unwrap();

                if game_tx
                    .send(GameCommand::Action {
                        user_id,
                        action: deserialized_action,
                    })
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    let _ = game.tx.send(GameCommand::Disconnected { user_id }).await;
    tracing::info!("User {} disconnected", user_id);
}
