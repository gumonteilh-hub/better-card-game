use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum_macros::debug_handler;
use back::{
    self, PublicGameState, UserDeck,
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
    routing::{any, get, post},
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
    matchmaking_queue: Arc<Mutex<Vec<MatchmakingPlayer>>>,
}

struct MatchmakingPlayer {
    user_id: Uuid,
    deck: UserDeck,
    tx: mpsc::Sender<MatchmakingMessage>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
enum MatchmakingClientMessage {
    JoinQueue { deck: UserDeck },
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

#[derive(Clone)]
struct GameHandle {
    tx: mpsc::Sender<GameCommand>,
}

#[derive(Debug)]
enum GameCommand {
    Connected {
        user_id: Uuid,
        ws_tx: mpsc::Sender<ServerMessage>,
    },

    Action {
        user_id: Uuid,
        action: PlayerActionCommand,
    },

    Disconnected {
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

async fn create_pvp_game(
    state: &Arc<AppState>,
    player_a_id: Uuid,
    deck_a: UserDeck,
    player_b_id: Uuid,
    deck_b: UserDeck,
) -> Uuid {
    // Récupérer collections
    let collection_a = back::get_collection(deck_a.archetype);
    let collection_b = back::get_collection(deck_b.archetype);

    // Créer game (PvP, pas vs_ia)
    let mut game =
        back::Game::new(deck_a, deck_b, collection_a, collection_b).expect("Failed to create game");

    game.vs_ia = false; // PvP mode
    game.compute_commands()
        .expect("Failed to compute initial commands");

    let game_id = game.game_id;

    // Spawn game task
    let (tx, rx) = mpsc::channel::<GameCommand>(100);
    tokio::spawn(game_task(
        game_id,
        game,
        player_a_id,
        player_b_id,
        rx,
        state.clone(),
    ));

    state.games.insert(game_id, GameHandle { tx });
    state.current_live_games.insert(player_a_id, game_id);
    state.current_live_games.insert(player_b_id, game_id);

    tracing::info!(
        "PvP game {} created: {} vs {}",
        game_id,
        player_a_id,
        player_b_id
    );

    game_id
}

#[debug_handler]
async fn handle_matchmaking(
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

    // Vérifier si l'utilisateur a déjà une partie en cours
    if let Some(existing_game_id) = state.current_live_games.get(&user_id) {
        let game_id = *existing_game_id.value();
        tracing::info!("User {} already has an active game: {}", user_id, game_id);

        // Envoyer immédiatement GameFound
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

                    break; // Fermer connexion matchmaking
                } else {
                    // En attente
                    let _ = tx_clone.send(MatchmakingMessage::Waiting).await;
                }
            }
        }

        // Connexion fermée → retirer de la queue si toujours présent
        state_clone
            .matchmaking_queue
            .lock()
            .unwrap()
            .retain(|p| p.user_id != user_id);

        tracing::info!("User {} left matchmaking", user_id);
    });

    // Attendre fin des tasks
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => {
            drop(tx); // Fermer le channel pour que send_task se termine
            let _ = send_task.await; // Attendre que send_task finisse d'envoyer tous les messages
        },
    }

    tracing::info!("Matchmaking socket closed for user {}", user_id);
}

#[debug_handler]
async fn handle(
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

async fn game_task(
    game_id: Uuid,
    initial_game: back::Game,
    player_a_id: Uuid,
    player_b_id: Uuid,
    mut rx: mpsc::Receiver<GameCommand>,
    app_state: Arc<AppState>,
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

    'main_loop: while let Some(cmd) = rx.recv().await {
        match cmd {
            GameCommand::Connected { user_id, ws_tx } => {
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

                broadcast_to_all(&state, ServerMessage::Message("Player joined".to_string())).await;
            }

            GameCommand::Action { user_id, action } => {
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
                        | Action::RefreshMana { .. } => {
                            broadcast_to_all(&state, ServerMessage::Action(action)).await;
                        }
                        Action::Win { .. } => {
                            broadcast_to_all(&state, ServerMessage::Action(action)).await;
                            break 'main_loop;
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

            GameCommand::Disconnected { user_id } => {
                tracing::info!("Player {} disconnected from game {}", user_id, game_id);
                state.player_channels.remove(&user_id);

                broadcast_to_all(&state, ServerMessage::Message("Player left".to_string())).await;
            }
        }
    }
    tracing::info!("Game {} finished, cleaning up state", game_id);
    app_state.current_live_games.remove(&player_a_id);
    app_state.current_live_games.remove(&player_b_id);
    app_state.games.remove(&game_id);
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
    if let Some(tx) = state.player_channels.get(&user_id)
        && tx.try_send(msg).is_err()
    {
        tracing::warn!("Failed to send to player {}", user_id);
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
        matchmaking_queue: Arc::new(Mutex::new(Vec::new())),
        current_live_games: DashMap::new(),
        games: DashMap::new(),
    });

    let game_routes = Router::new().route("/{game_id}/{user_id}", any(handle));
    let matchmaking_routes = Router::new().route("/{user_id}", any(handle_matchmaking));

    let app = Router::new()
        .route("/collection", post(collection))
        //.route("/start", post(start_game))
        .route("/user/{user_id}", get(find_current_game))
        .nest("/matchmaking", matchmaking_routes)
        .nest("/game", game_routes)
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
async fn find_current_game(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let game_id = state
        .current_live_games
        .get(&user_id)
        .map(|entry| *entry.value());
    Ok(Json(serde_json::json!(game_id)))
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

    tokio::spawn(game_task(
        game_id,
        game_state,
        player_1_id,
        player_2_id,
        rx,
        state.clone(),
    ));

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
