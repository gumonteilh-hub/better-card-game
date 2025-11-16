#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::too_many_arguments)]

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

fn append_game_view_updates(
    game_state: &Game,
    player: PlayerId,
    actions: &mut Vec<Action>,
) -> Result<()> {
    let player_game_view = PublicGameState::new(game_state, player)?;
    actions.push(Action::UpdateGameView {
        player,
        game: player_game_view,
    });

    let opponent = game_state.get_opponent(&player)?;

    let opponent_game_view = PublicGameState::new(game_state, opponent.player_id)?;
    actions.push(Action::UpdateGameView {
        player: opponent.player_id,
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
    let mut actions = game::user_actions::play_monster::play_monster(
        &mut game_state,
        player,
        card_id,
        position,
        targets,
    )?;
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
    game::user_actions::play_spell::play_spell(&mut game_state, player, card_id, targets)?;
    let mut actions = game_state.compute_commands()?;
    append_game_view_updates(&game_state, player, &mut actions)?;
    Ok((actions, game_state))
}

pub fn end_turn(mut game_state: Game, player: PlayerId) -> Result<(Vec<Action>, Game)> {
    let mut actions = game::user_actions::end_turn::end_turn(&mut game_state, player)?;
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
    game::user_actions::attack::attack(&mut game_state, player, initiator, target)?;
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
    let mut actions =
        game::user_actions::move_card::move_card(&mut game_state, player, card_id, position)?;
    let compute_actions = game_state.compute_commands()?;
    actions.extend(compute_actions);
    append_game_view_updates(&game_state, player, &mut actions)?;
    Ok((actions, game_state))
}
