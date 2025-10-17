#![allow(dead_code)]
#![allow(unused_variables)]

use crate::collection::Faction;
pub use crate::game::Game;
use crate::game::action::Action;
use crate::game::card::CardTemplate;
use crate::game::view::PublicGameState;

mod collection;
pub mod error;
mod game;
mod ia;
mod template;

use error::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GameViewResponse {
    actions: Vec<Action>,
    game_view: PublicGameState,
}

#[derive(Debug, Deserialize)]
pub struct UserDeck {
    pub cards: Vec<CardTemplate>,
    pub faction: Faction,
}

pub fn get_collection(faction: Faction) -> Vec<CardTemplate> {
    collection::get_collection(faction)
}

pub fn start_game(deck: UserDeck) -> Result<(GameViewResponse, Game)> {
    let mut game_state = Game::new(deck, collection::get_ia_deck());

    let actions = game_state.compute_commands()?;
    let game_view = PublicGameState::new(&game_state)?;

    Ok((GameViewResponse { actions, game_view }, game_state))
}

pub fn play_card(
    game_sate: &mut Game,
    card_id: usize,
    position: usize,
) -> Result<GameViewResponse> {
    game_sate.play_card(card_id, position)?;
    let actions = game_sate.compute_commands()?;
    let game_view = PublicGameState::new(game_sate)?;

    let response = GameViewResponse { actions, game_view };
    Ok(response)
}

pub fn end_turn(game_sate: &mut Game) -> Result<GameViewResponse> {
    let mut actions = game_sate.next_turn()?;
    let mut other_actions = game_sate.compute_commands()?;
    actions.append(&mut other_actions);
    let game_view = PublicGameState::new(game_sate)?;

    Ok(GameViewResponse { actions, game_view })
}

pub fn attack(game_sate: &mut Game, initiator: usize, target: usize) -> Result<GameViewResponse> {
    game_sate.attack(initiator, target)?;
    let actions = game_sate.compute_commands()?;
    let game_view = PublicGameState::new(game_sate)?;

    Ok(GameViewResponse { actions, game_view })
}
