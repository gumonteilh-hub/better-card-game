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
    game::action::Action,
};

pub mod collection;
pub mod error;
mod game;
mod ia;

use error::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameViewResponse {
    actions: Vec<Action>,
    game_view: PublicGameState,
}

#[derive(Debug, Deserialize)]
pub struct UserDeck {
    pub cards: Vec<TemplateId>,
    pub archetype: Archetype,
}

pub fn get_collection(archetype: Archetype) -> Vec<CardTemplate> {
    collection::get_collection(archetype)
}

pub fn start_game(deck: UserDeck) -> Result<Game> {
    let ia_deck = collection::get_ia_deck();
    let ia_archetype = ia_deck.archetype;
    let player_archetype = deck.archetype;
    let mut game_state = Game::new(
        deck,
        ia_deck,
        get_collection(player_archetype),
        get_collection(ia_archetype),
    )?;

    game_state.compute_commands()?;

    Ok(game_state)
}

pub fn play_monster(
    game_state: &mut Game,
    card_id: usize,
    position: usize,
) -> Result<GameViewResponse> {
    game_state.play_monster(card_id, position)?;
    let actions = game_state.compute_commands()?;
    let game_view = PublicGameState::new(game_state)?;

    let response = GameViewResponse { actions, game_view };
    Ok(response)
}

pub fn play_spell(game_state: &mut Game, card_id: usize) -> Result<GameViewResponse> {
    game_state.play_spell(card_id)?;
    let actions = game_state.compute_commands()?;
    let game_view = PublicGameState::new(game_state)?;

    let response = GameViewResponse { actions, game_view };
    Ok(response)
}

pub fn end_turn(game_state: &mut Game) -> Result<GameViewResponse> {
    let mut actions = game_state.next_turn()?;
    let mut other_actions = game_state.compute_commands()?;
    actions.append(&mut other_actions);
    let game_view = PublicGameState::new(game_state)?;

    Ok(GameViewResponse { actions, game_view })
}

pub fn attack(game_state: &mut Game, initiator: usize, target: usize) -> Result<GameViewResponse> {
    game_state.attack(initiator, target)?;
    let actions = game_state.compute_commands()?;
    let game_view = PublicGameState::new(game_state)?;

    Ok(GameViewResponse { actions, game_view })
}

pub fn move_card(
    game_state: &mut Game,
    card_id: usize,
    position: usize,
) -> Result<GameViewResponse> {
    game_state.move_card(card_id, position)?;
    let actions = game_state.compute_commands()?;
    let game_view = PublicGameState::new(game_state)?;

    Ok(GameViewResponse { actions, game_view })
}
