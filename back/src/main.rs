use std::sync::Arc;

use axum_macros::debug_handler;
use back::{self, error::Error};

use axum::{
    Json, Router,
    extract::{FromRequest, Request, State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use serde::de::DeserializeOwned;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    game_states: Mutex<Vec<back::Game>>,
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
        game_states: Mutex::new(Vec::new()),
    });

    let app = Router::new()
        .route("/start", post(start_game))
        .with_state(shared_state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9999")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn start_game(
    State(state): State<Arc<AppState>>,
    LoggedJson(payload): LoggedJson<back::UserDeck>,
) -> ApiResult<Json<back::GameViewResponse>> {
    tracing::info!("Received start_game request with deck: {:?}", payload);
    let (game_view, game_state) = back::start_game(payload)?;
    state.game_states.lock().await.push(game_state);
    tracing::info!("Game started successfully");
    return Ok(Json(game_view));
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
