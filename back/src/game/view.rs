use serde::Serialize;

use super::card::CardInstance;
use super::types::Location;
use super::Game;
use crate::{
    collection::Faction, error::{Error, Result}, game::types::PlayerId
};

#[derive(Serialize)]
pub struct PlayerInfo {
    pub id: usize,
    pub name: String,
    pub hp: usize,
    pub faction: Faction,
}

#[derive(Serialize)]
pub struct PublicGameState {
    pub ennemy_field: Vec<CardInstance>,
    pub ennemy_hand_size: usize,
    pub ennemy_mana: usize,
    pub ennemy_info: PlayerInfo,
    pub player_info: PlayerInfo,
    pub ennemy_base_mana: usize,
    pub ennemy_deck_size: usize,
    pub deck_size: usize,
    pub field: Vec<CardInstance>,
    pub hand: Vec<CardInstance>,
    pub turn: usize,
    pub mana: usize,
    pub base_mana: usize,
    pub winner_id: Option<PlayerId>,
}

impl PublicGameState {
    pub fn new(game_state: &Game) -> Result<Self> {
        let player_a = game_state
            .players
            .get(&game_state.player_id_a)
            .ok_or_else(|| {
                Error::Game(format!(
                    "Player A with id {} not found",
                    game_state.player_id_a
                ))
            })?;
        let player_b = game_state
            .players
            .get(&game_state.player_id_b)
            .ok_or_else(|| {
                Error::Game(format!(
                    "Player B with id {} not found",
                    game_state.player_id_b
                ))
            })?;

        let player_entities = game_state
            .entities
            .values()
            .filter(|e| e.owner == game_state.player_id_a);
        let ennemy_entities = game_state
            .entities
            .values()
            .filter(|e| e.owner == game_state.player_id_b);
        Ok(Self {
            player_info: PlayerInfo {
                id: player_a.player_id,
                name: "PlayerTest".into(),
                hp: player_a.hp,
                faction: player_a.faction,
            },
            ennemy_info: PlayerInfo {
                id: player_b.player_id,
                name: "EnnemyIA".into(),
                hp: player_b.hp,
                faction: player_b.faction,
            },
            ennemy_field: ennemy_entities
                .clone()
                .filter(|e| matches!(e.location, Location::Field(_)))
                .cloned()
                .collect(),
            ennemy_hand_size: ennemy_entities
                .clone()
                .filter(|e| e.location == Location::Hand)
                .count(),
            ennemy_mana: player_b.mana,
            ennemy_base_mana: player_b.base_mana,
            ennemy_deck_size: ennemy_entities
                .clone()
                .filter(|e| e.location == Location::Deck)
                .count(),
            field: player_entities
                .clone()
                .filter(|e| matches!(e.location, Location::Field(_)))
                .cloned()
                .collect(),
            hand: player_entities
                .clone()
                .filter(|e| e.location == Location::Hand)
                .cloned()
                .collect(),
            turn: game_state.turn,
            mana: player_a.mana,
            base_mana: player_a.base_mana,
            deck_size: player_entities
                .filter(|e| e.location == Location::Deck)
                .count(),
            winner_id: game_state.winner_id,
        })
    }

    pub fn serialize_to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Error::from)
    }
}
