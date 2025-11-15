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

pub fn play_monster(
    game_state: &mut Game,
    player: PlayerId,
    card_id: usize,
    position: usize,
    targets: Option<Vec<InstanceId>>,
) -> Result<Vec<Action>> {
    let mut actions = game_state.play_monster(player, card_id, position, targets)?;
    let compute_actions = game_state.compute_commands()?;
    actions.extend(compute_actions);
    let player_game_view = PublicGameState::new(game_state, player)?;
    actions.push(Action::UpdateGameView {
        player,
        game: player_game_view,
    });
    let oponent = game_state.players.keys().find(|p| **p != player).unwrap();
    let oponent_game_view = PublicGameState::new(game_state, *oponent)?;
    actions.push(Action::UpdateGameView {
        player: *oponent,
        game: oponent_game_view,
    });
    Ok(actions)
}

pub fn play_spell(
    game_state: &mut Game,
    player: PlayerId,
    card_id: usize,
    targets: Option<Vec<InstanceId>>,
) -> Result<Vec<Action>> {
    game_state.play_spell(player, card_id, targets)?;
    let mut actions = game_state.compute_commands()?;
    let player_game_view = PublicGameState::new(game_state, player)?;
    actions.push(Action::UpdateGameView {
        player,
        game: player_game_view,
    });
    let oponent = game_state.players.keys().find(|p| **p != player).unwrap();
    let oponent_game_view = PublicGameState::new(game_state, *oponent)?;
    actions.push(Action::UpdateGameView {
        player: *oponent,
        game: oponent_game_view,
    });
    Ok(actions)
}

pub fn end_turn(game_state: &mut Game, player: PlayerId) -> Result<Vec<Action>> {
    let mut actions = game_state.end_turn(player)?;
    let other_actions = game_state.compute_commands()?;
    actions.extend(other_actions);
    let player_game_view = PublicGameState::new(game_state, player)?;
    actions.push(Action::UpdateGameView {
        player,
        game: player_game_view,
    });
    let oponent = game_state.players.keys().find(|p| **p != player).unwrap();
    let oponent_game_view = PublicGameState::new(game_state, *oponent)?;
    actions.push(Action::UpdateGameView {
        player: *oponent,
        game: oponent_game_view,
    });
    Ok(actions)
}

pub fn attack(
    game_state: &mut Game,
    player: PlayerId,
    initiator: usize,
    target: usize,
) -> Result<Vec<Action>> {
    game_state.attack(player, initiator, target)?;
    let mut actions = game_state.compute_commands()?;
    let player_game_view = PublicGameState::new(game_state, player)?;
    actions.push(Action::UpdateGameView {
        player,
        game: player_game_view,
    });
    let oponent = game_state.players.keys().find(|p| **p != player).unwrap();
    let oponent_game_view = PublicGameState::new(game_state, *oponent)?;
    actions.push(Action::UpdateGameView {
        player: *oponent,
        game: oponent_game_view,
    });
    Ok(actions)
}

pub fn move_card(
    game_state: &mut Game,
    player: PlayerId,
    card_id: usize,
    position: usize,
) -> Result<Vec<Action>> {
    game_state.move_card(player, card_id, position)?;
    let mut actions = game_state.compute_commands()?;
    let player_game_view = PublicGameState::new(game_state, player)?;
    actions.push(Action::UpdateGameView {
        player,
        game: player_game_view,
    });
    let oponent = game_state.players.keys().find(|p| **p != player).unwrap();
    let oponent_game_view = PublicGameState::new(game_state, *oponent)?;
    actions.push(Action::UpdateGameView {
        player: *oponent,
        game: oponent_game_view,
    });
    Ok(actions)
}
