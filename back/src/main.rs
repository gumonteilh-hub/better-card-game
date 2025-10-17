use std::sync::Arc;

use axum_macros::debug_handler;
use back::{self, error::Error};

use axum::{
    extract::{Path, State}, routing::{get, post}, Json, Router
};
use tokio::sync::Mutex;

struct AppState {
    game_states: Mutex<Vec<back::Game>>,
}

type ApiResult<T> = Result<T, Error>;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {
        game_states: Mutex::new(Vec::new()),
    });

    let app = Router::new()
        .route("/{user_id}", get(hello_word))
        .route("/start", post(start_game))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9999")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
    println!("Hello, world!");
}

async fn hello_word(Path(user_id): Path<u32>) -> String {
    format!("Hello world: {user_id}")
}

#[debug_handler]
async fn start_game(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<back::UserDeck>,
) -> ApiResult<Json<back::GameViewResponse>> {
    let (game_view, game_state) = back::start_game(payload)?;
    state.game_states.lock().await.push(game_state);
    return Ok(Json(game_view));
}
