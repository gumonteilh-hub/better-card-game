use std::{collections::HashMap, sync::Arc};

use back::{
    PublicGameState, UserDeck,
    collection::get_ia_deck,
    game::{action::Action, types::PlayerId},
    get_collection,
};
use serde::Serialize;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{AppState, server::handle_game::PlayerActionCommand};

#[derive(Clone)]
pub struct GameHandle {
    pub tx: mpsc::Sender<GameCommand>,
}

#[derive(Debug)]
pub enum GameCommand {
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

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum ServerMessage {
    Action(back::game::action::Action),
    Error(String),
    Message(String),
}

pub struct GameState {
    game: back::Game,
    player_id_turn: Uuid,
    user_id_player_id_mapping: HashMap<Uuid, PlayerId>,
    player_channels: HashMap<Uuid, mpsc::Sender<ServerMessage>>,
}

pub async fn create_game_vs_ia(state: &Arc<AppState>, player_id: Uuid, deck: UserDeck) -> Uuid {
    let ia_deck = get_ia_deck();

    let ia_id = Uuid::new_v4();

    let mut game = back::Game::new(
        deck.clone(),
        ia_deck.clone(),
        get_collection(deck.archetype),
        get_collection(ia_deck.archetype),
        true,
    )
    .expect("Failed to create game");

    game.compute_commands()
        .expect("Failed to compute initial commands");

    let game_id = game.game_id;

    let (tx, rx) = mpsc::channel::<GameCommand>(100);
    tokio::spawn(game_task(
        game_id,
        game,
        player_id,
        ia_id,
        rx,
        state.clone(),
    ));

    state.games.insert(game_id, GameHandle { tx });
    state.current_live_games.insert(player_id, game_id);

    tracing::info!("PvE game {} created: {} vs IA", game_id, player_id,);

    game_id
}

pub async fn create_pvp_game(
    state: &Arc<AppState>,
    player_a_id: Uuid,
    deck_a: UserDeck,
    player_b_id: Uuid,
    deck_b: UserDeck,
) -> Uuid {
    let collection_a = back::get_collection(deck_a.archetype);
    let collection_b = back::get_collection(deck_b.archetype);

    let mut game = back::Game::new(deck_a, deck_b, collection_a, collection_b, false)
        .expect("Failed to create game");

    game.vs_ia = false;
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

pub async fn game_task(
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
                let cloned_game_state = state.game.clone();

                let result = match action {
                    PlayerActionCommand::PlayMonster {
                        card_id,
                        position,
                        targets,
                    } => back::play_monster(
                        cloned_game_state,
                        *player_id,
                        card_id,
                        position,
                        targets,
                    ),
                    PlayerActionCommand::PlaySpell { card_id, targets } => {
                        back::play_spell(cloned_game_state, *player_id, card_id, targets)
                    }
                    PlayerActionCommand::EndTurn => back::end_turn(cloned_game_state, *player_id),
                    PlayerActionCommand::Attack { initiator, target } => {
                        back::attack(cloned_game_state, *player_id, initiator, target)
                    }
                    PlayerActionCommand::Move { card_id, position } => {
                        back::move_card(cloned_game_state, *player_id, card_id, position)
                    }
                };

                let actions = match result {
                    Ok((actions, new_state)) => {
                        state.game = new_state;
                        actions
                    }
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

async fn broadcast_to_player(state: &GameState, user_id: Uuid, msg: ServerMessage) {
    if let Some(tx) = state.player_channels.get(&user_id)
        && tx.try_send(msg).is_err()
    {
        tracing::warn!("Failed to send to player {}", user_id);
    }
}
