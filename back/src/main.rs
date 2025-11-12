use std::sync::{Arc, Mutex};

use axum_macros::debug_handler;
use back::{self, collection::Archetype, error::Error};

use crate::server::{
    handle_game::handle_game,
    handle_matchmaking::{MatchmakingPlayer, handle_matchmaking},
    initialize::{GameHandle, create_game_vs_ia},
};
use axum::{
    Json, Router,
    extract::{FromRequest, Path, Request, State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{any, get, post},
};
use dashmap::DashMap;
use serde::{Serialize, de::DeserializeOwned};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

mod server;

struct AppState {
    current_live_games: DashMap<Uuid, Uuid>,
    games: DashMap<Uuid, GameHandle>,
    matchmaking_queue: Arc<Mutex<Vec<MatchmakingPlayer>>>,
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

    let game_routes = Router::new().route("/{game_id}/{user_id}", any(handle_game));
    let matchmaking_routes = Router::new().route("/{user_id}", any(handle_matchmaking));

    let app = Router::new()
        .route("/collection", post(collection))
        .route("/ia/{user_id}", post(start_game_vs_ia))
        .route("/user/{user_id}", get(find_current_game))
        .nest("/ws/matchmaking", matchmaking_routes)
        .nest("/ws/game", game_routes)
        .with_state(shared_state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9999")
        .await
        .unwrap();

    tracing::info!("Server listening on http://127.0.0.1:9999");
    axum::serve(listener, app).await.unwrap();
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
async fn start_game_vs_ia(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    LoggedJson(payload): LoggedJson<back::UserDeck>,
) -> ApiResult<Json<serde_json::Value>> {
    tracing::info!("Received start_game request with deck: {:?}", payload);

    let game_id = create_game_vs_ia(&state, user_id, payload).await;

    Ok(Json(serde_json::json!(StartGameInfo {
        user_id: user_id.to_string(),
        game_id: game_id.to_string(),
    })))
}

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
