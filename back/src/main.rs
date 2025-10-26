use std::{collections::HashMap, sync::Arc};

use axum_macros::debug_handler;
use back::{self, error::Error};

use axum::{
    Json, Router,
    extract::{FromRequest, Path, Request, State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::de::DeserializeOwned;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

struct AppState {
    game_states: Mutex<HashMap<Uuid, back::Game>>,
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
        game_states: Mutex::new(HashMap::new()),
    });

    let api = Router::new()
        .route("/{game_id}", get(game_state))
        .route("/{game_id}/attack/{initiator_id}/{target_id}", post(attack))
        .route("/{game_id}/move/{card_id}/{target_pos}", post(move_card))
        .route("/{game_id}/play_card/{card_id}/{position}", post(play_card))
        .route("/{game_id}/end_turn", post(end_turn));

    let app = Router::new()
        .route("/collection/{faction}", get(collection))
        .route("/start", post(start_game))
        .nest("/game", api)
        .with_state(shared_state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9999")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn collection(
    Path(faction): Path<back::Faction>,
) -> ApiResult<Json<Vec<back::CardTemplate>>> {
    tracing::info!(
        "Received get_collection request with faction: {:?}",
        faction
    );
    return Ok(Json(back::get_collection(faction)));
}

#[debug_handler]
async fn start_game(
    State(state): State<Arc<AppState>>,
    LoggedJson(payload): LoggedJson<back::UserDeck>,
) -> ApiResult<Json<String>> {
    tracing::info!("Received start_game request with deck: {:?}", payload);
    let game_state = back::start_game(payload)?;
    let game_id = game_state.game_id.to_string();
    state
        .game_states
        .lock()
        .await
        .insert(game_state.game_id, game_state);
    tracing::info!("Game started successfully");
    return Ok(Json(game_id));
}

#[debug_handler]
async fn play_card(
    State(state): State<Arc<AppState>>,
    Path((game_id, card_id, position)): Path<(Uuid, usize, usize)>,
) -> ApiResult<Json<back::GameViewResponse>> {
    tracing::info!(
        "Received play_card request for game: {}, with card_id: {}, and position: {}",
        game_id,
        card_id,
        position
    );
    match state.game_states.lock().await.get_mut(&game_id) {
        Some(mut game) => {
            let game_view = back::play_card(&mut game, card_id, position)?;
            tracing::info!("Play card performed successfully");
            Ok(Json(game_view))
        }
        None => Err(back::error::Error::GameNotStarted),
    }
}

#[debug_handler]
async fn game_state(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<Uuid>,
) -> ApiResult<Json<back::PublicGameState>> {
    tracing::info!("Received get game_state  request for game: {}", game_id);
    match state.game_states.lock().await.get(&game_id) {
        Some(game) => {
            let game_view = back::PublicGameState::new(game)?;
            Ok(Json(game_view))
        }
        None => Err(back::error::Error::GameNotStarted),
    }
}

#[debug_handler]
async fn attack(
    State(state): State<Arc<AppState>>,
    Path((game_id, initiator_id, target_id)): Path<(Uuid, usize, usize)>,
) -> ApiResult<Json<back::GameViewResponse>> {
    tracing::info!(
        "Received attack request for game: {}, with initiator: {}, and target: {}",
        game_id,
        initiator_id,
        target_id
    );
    match state.game_states.lock().await.get_mut(&game_id) {
        Some(mut game) => {
            let game_view = back::attack(&mut game, initiator_id, target_id)?;
            tracing::info!("Attack performed successfully");
            Ok(Json(game_view))
        }
        None => Err(back::error::Error::GameNotStarted),
    }
}

#[debug_handler]
async fn move_card(
    State(state): State<Arc<AppState>>,
    Path((game_id, card_id, target_pos)): Path<(Uuid, usize, usize)>,
) -> ApiResult<Json<back::GameViewResponse>> {
    tracing::info!(
        "Received move_card request for game: {}, with card: {}, moving to position: {}",
        game_id,
        card_id,
        target_pos
    );
    match state.game_states.lock().await.get_mut(&game_id) {
        Some(mut game) => {
            let game_view = back::move_card(&mut game, card_id, target_pos)?;
            tracing::info!("Move performed successfully");
            Ok(Json(game_view))
        }
        None => Err(back::error::Error::GameNotStarted),
    }
}

#[debug_handler]
async fn end_turn(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<Uuid>,
) -> ApiResult<Json<back::GameViewResponse>> {
    tracing::info!("Received end turn request for game: {}", game_id,);
    match state.game_states.lock().await.get_mut(&game_id) {
        Some(mut game) => {
            let game_view = back::end_turn(&mut game)?;
            tracing::info!("End turn performed successfully");
            Ok(Json(game_view))
        }
        None => Err(back::error::Error::GameNotStarted),
    }
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
