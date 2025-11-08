use std::{collections::HashMap, sync::Arc};

use axum_macros::debug_handler;
use back::{
    self, PublicGameState,
    collection::Archetype,
    error::Error,
    game::{action::Action, types::PlayerId},
};

use axum::{
    Json, Router,
    extract::{
        FromRequest, Path, Request, State, WebSocketUpgrade,
        rejection::JsonRejection,
        ws::{Message, WebSocket},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{any, post},
};
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::sync::mpsc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

struct AppState {
    current_live_games: DashMap<Uuid, Uuid>,
    games: DashMap<Uuid, GameHandle>,
}

#[derive(Clone)]
struct GameHandle {
    tx: mpsc::Sender<GameCommand>,
}

#[derive(Debug)]
enum GameCommand {
    PlayerConnected {
        user_id: Uuid,
        ws_tx: mpsc::Sender<ServerMessage>,
    },

    PlayerAction {
        user_id: Uuid,
        action: PlayerActionCommand,
    },

    PlayerDisconnected {
        user_id: Uuid,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
enum PlayerActionCommand {
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

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
enum ServerMessage {
    Action(back::game::action::Action),
    Error(String),
    Message(String),
}

struct GameState {
    game: back::Game,
    player_id_turn: Uuid,
    user_id_player_id_mapping: HashMap<Uuid, PlayerId>,
    player_channels: HashMap<Uuid, mpsc::Sender<ServerMessage>>,
}

#[debug_handler]
async fn handle(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
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
        .send(GameCommand::PlayerConnected { user_id, ws_tx: tx })
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
                    .send(GameCommand::PlayerAction {
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

    let _ = game
        .tx
        .send(GameCommand::PlayerDisconnected { user_id })
        .await;
    tracing::info!("User {} disconnected", user_id);
}

async fn game_task(
    game_id: Uuid,
    initial_game: back::Game,
    player_a_id: Uuid,
    player_b_id: Uuid,
    mut rx: mpsc::Receiver<GameCommand>,
) {
    tracing::info!("Game task started for game {}", game_id);

    let mut player_map: HashMap<Uuid, PlayerId> = HashMap::new();
    player_map.insert(player_a_id, 0);
    player_map.insert(player_b_id, 1);

    let mut state = GameState {
        game: initial_game,
        user_id_player_id_mapping: player_map,
        player_id_turn: player_a_id,
        player_channels: HashMap::new(),
    };

    while let Some(cmd) = rx.recv().await {
        match cmd {
            GameCommand::PlayerConnected { user_id, ws_tx } => {
                tracing::info!("Player {} connected to game {}", user_id, game_id);
                state.player_channels.insert(user_id, ws_tx);

                let player_id = state.user_id_player_id_mapping.get(&user_id).unwrap();

                broadcast_to_player(
                    &state,
                    user_id,
                    ServerMessage::Action(Action::UpdateGameView {
                        player: *player_id,
                        game: PublicGameState::new(&state.game, *player_id).unwrap(),
                    }),
                )
                .await;

                broadcast_to_all(&state, ServerMessage::Message(format!("Player joined"))).await;
            }

            GameCommand::PlayerAction { user_id, action } => {
                tracing::info!("Player {} wants to do: {:?}", user_id, action);

                tracing::info!("{}", state.player_id_turn);

                if state.player_id_turn != user_id {
                    broadcast_to_player(
                        &state,
                        user_id,
                        ServerMessage::Error("Not your turn!".into()),
                    )
                    .await;
                    continue;
                }

                let player_id = state.user_id_player_id_mapping.get(&user_id).unwrap();

                let result_actions = match action {
                    PlayerActionCommand::PlayMonster { card_id, position } => {
                        back::play_monster(&mut state.game, *player_id, card_id, position)
                    }
                    PlayerActionCommand::PlaySpell { card_id } => {
                        back::play_spell(&mut state.game, *player_id, card_id)
                    }
                    PlayerActionCommand::EndTurn => state.game.end_turn(*player_id),
                    PlayerActionCommand::Attack { initiator, target } => {
                        back::attack(&mut state.game, *player_id, initiator, target)
                    }
                    PlayerActionCommand::Move { card_id, position } => {
                        back::move_card(&mut state.game, *player_id, card_id, position)
                    }
                };

                let actions = match result_actions {
                    Ok(actions) => actions,
                    Err(error) => {
                        broadcast_to_player(
                            &state,
                            user_id,
                            ServerMessage::Error(error.to_string()),
                        )
                        .await;
                        continue;
                    }
                };

                for action in actions {
                    match action {
                        Action::UpdateGameView { player, .. }
                        | Action::Draw { player, .. }
                        | Action::EnemyDraw { player } => {
                            let (user_id, _) = state
                                .user_id_player_id_mapping
                                .iter()
                                .find(|(_, player_id)| **player_id == player)
                                .unwrap();

                            broadcast_to_player(&state, *user_id, ServerMessage::Action(action))
                                .await;
                        }
                        Action::Boost { .. }
                        | Action::IncreaseMaxMana { .. }
                        | Action::BurnCard { .. }
                        | Action::Heal { .. }
                        | Action::Destroy { .. }
                        | Action::ReceiveDamage { .. }
                        | Action::Summon { .. }
                        | Action::Attack { .. }
                        | Action::TriggerOnPlay { .. }
                        | Action::TriggerOnDeath { .. }
                        | Action::TriggerOnAttack { .. }
                        | Action::Win { .. }
                        | Action::RefreshMana { .. } => {
                            broadcast_to_all(&state, ServerMessage::Action(action)).await;
                        }
                        Action::StartTurn(player) => {
                            let (user_id, _) = state
                                .user_id_player_id_mapping
                                .iter()
                                .find(|(_, player_id)| **player_id == player)
                                .unwrap();
                            state.player_id_turn = *user_id
                        }
                    }
                }
            }

            GameCommand::PlayerDisconnected { user_id } => {
                tracing::info!("Player {} disconnected from game {}", user_id, game_id);
                state.player_channels.remove(&user_id);

                broadcast_to_all(&state, ServerMessage::Message(format!("Player left"))).await;
            }
        }
    }

    tracing::info!("Game task ended for game {}", game_id);
}

async fn broadcast_to_all(state: &GameState, msg: ServerMessage) {
    for (user_id, tx) in &state.player_channels {
        if tx.try_send(msg.clone()).is_err() {
            tracing::warn!("Failed to send to player {}", user_id);
        }
    }
}

/// Envoie un message à un joueur spécifique
async fn broadcast_to_player(state: &GameState, user_id: Uuid, msg: ServerMessage) {
    if let Some(tx) = state.player_channels.get(&user_id) {
        if tx.try_send(msg).is_err() {
            tracing::warn!("Failed to send to player {}", user_id);
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "back=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let shared_state = Arc::new(AppState {
        current_live_games: DashMap::new(),
        games: DashMap::new(),
    });

    let ws_routes = Router::new().route("/{user_id}", any(handle));

    let app = Router::new()
        .route("/collection", post(collection))
        .route("/start", post(start_game))
        .nest("/game", ws_routes)
        .with_state(shared_state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9999")
        .await
        .unwrap();

    tracing::info!("Server listening on http://127.0.0.1:9999");
    axum::serve(listener, app).await.unwrap();
}

// ============================================================================
// HTTP REST ENDPOINTS
// ============================================================================

#[debug_handler]
async fn collection(
    LoggedJson(payload): LoggedJson<Archetype>,
) -> ApiResult<Json<Vec<back::collection::types::CardTemplate>>> {
    tracing::info!(
        "Received get_collection request with archetype: {:?}",
        payload
    );

    Ok(Json(back::get_collection(payload)))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StartGameInfo {
    game_id: String,
    user_id: String,
}

#[debug_handler]
async fn start_game(
    State(state): State<Arc<AppState>>,
    LoggedJson(payload): LoggedJson<back::UserDeck>,
) -> ApiResult<Json<serde_json::Value>> {
    tracing::info!("Received start_game request with deck: {:?}", payload);

    let game_state = back::start_game(payload)?;
    let game_id = game_state.game_id;

    let player_1_id = Uuid::new_v4();
    let player_2_id = Uuid::new_v4();

    let (tx, rx) = mpsc::channel::<GameCommand>(100);

    tokio::spawn(game_task(game_id, game_state, player_1_id, player_2_id, rx));

    state.games.insert(game_id, GameHandle { tx });

    state.current_live_games.insert(player_1_id, game_id);
    state.current_live_games.insert(player_2_id, game_id);

    tracing::info!("Game started successfully");

    Ok(Json(serde_json::json!(StartGameInfo {
        user_id: player_1_id.into(),
        game_id: game_id.into(),
    })))
}

struct LoggedJson<T>(pub T);

impl<S, T> FromRequest<S> for LoggedJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(LoggedJson(value)),
            Err(rejection) => {
                tracing::error!("Erreur de désérialisation JSON: {:?}", rejection);

                let error_message = match &rejection {
                    JsonRejection::JsonDataError(err) => {
                        format!(
                            "Erreur de parsing JSON: {}. Vérifiez la structure de votre JSON.",
                            err
                        )
                    }
                    JsonRejection::JsonSyntaxError(err) => {
                        format!("Syntaxe JSON invalide: {}", err)
                    }
                    JsonRejection::MissingJsonContentType(err) => {
                        format!("Header Content-Type manquant ou invalide: {}", err)
                    }
                    _ => format!("Erreur de désérialisation: {}", rejection),
                };

                tracing::error!("Détail de l'erreur: {}", error_message);

                Err((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(serde_json::json!({
                        "error": error_message
                    })),
                )
                    .into_response())
            }
        }
    }
}

type ApiResult<T> = Result<T, Error>;
