#![allow(dead_code)]
#![allow(unused_variables)]

pub use crate::collection::Race;
pub use crate::game::Game;
pub use crate::game::view::PublicGameState;
use crate::{
    collection::{
        Archetype,
        types::{CardTemplate, TemplateId},
    },
    game::{
        action::Action,
        types::{InstanceId, PlayerId},
    },
};

pub mod collection;
pub mod error;
pub mod game;
mod ia;

use error::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameViewResponse {
    actions: Vec<Action>,
    game_view: PublicGameState,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserDeck {
    pub cards: Vec<TemplateId>,
    pub archetype: Archetype,
}

pub fn get_collection(archetype: Archetype) -> Vec<CardTemplate> {
    collection::get_collection(archetype)
}

/// Helper function to append game view updates for both players to the actions vector
/// This eliminates code duplication across all public game functions
fn append_game_view_updates(
    game_state: &Game,
    player: PlayerId,
    actions: &mut Vec<Action>,
) -> Result<()> {
    // Create and push game view for the current player
    let player_game_view = PublicGameState::new(game_state, player)?;
    actions.push(Action::UpdateGameView {
        player,
        game: player_game_view,
    });

    // Find the opponent player
    let opponent = game_state
        .players
        .keys()
        .find(|p| **p != player)
        .ok_or_else(|| error::Error::Game(format!("Opponent not found for player {:?}", player)))?;

    // Create and push game view for the opponent
    let opponent_game_view = PublicGameState::new(game_state, *opponent)?;
    actions.push(Action::UpdateGameView {
        player: *opponent,
        game: opponent_game_view,
    });

    Ok(())
}

pub fn play_monster(
    mut game_state: Game,
    player: PlayerId,
    card_id: usize,
    position: usize,
    targets: Option<Vec<InstanceId>>,
) -> Result<(Vec<Action>, Game)> {
    let mut actions = game_state.play_monster(player, card_id, position, targets)?;
    let compute_actions = game_state.compute_commands()?;
    actions.extend(compute_actions);
    append_game_view_updates(&game_state, player, &mut actions)?;
    Ok((actions, game_state))
}

pub fn play_spell(
    mut game_state: Game,
    player: PlayerId,
    card_id: usize,
    targets: Option<Vec<InstanceId>>,
) -> Result<(Vec<Action>, Game)> {
    game_state.play_spell(player, card_id, targets)?;
    let mut actions = game_state.compute_commands()?;
    append_game_view_updates(&game_state, player, &mut actions)?;
    Ok((actions, game_state))
}

pub fn end_turn(mut game_state: Game, player: PlayerId) -> Result<(Vec<Action>, Game)> {
    let mut actions = game_state.end_turn(player)?;
    let other_actions = game_state.compute_commands()?;
    actions.extend(other_actions);
    append_game_view_updates(&game_state, player, &mut actions)?;
    Ok((actions, game_state))
}

pub fn attack(
    mut game_state: Game,
    player: PlayerId,
    initiator: usize,
    target: usize,
) -> Result<(Vec<Action>, Game)> {
    game_state.attack(player, initiator, target)?;
    let mut actions = game_state.compute_commands()?;
    append_game_view_updates(&game_state, player, &mut actions)?;
    Ok((actions, game_state))
}

pub fn move_card(
    mut game_state: Game,
    player: PlayerId,
    card_id: usize,
    position: usize,
) -> Result<(Vec<Action>, Game)> {
    game_state.move_card(player, card_id, position)?;
    let mut actions = game_state.compute_commands()?;
    append_game_view_updates(&game_state, player, &mut actions)?;
    Ok((actions, game_state))
}
