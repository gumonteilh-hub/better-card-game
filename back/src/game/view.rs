use std::collections::HashMap;

use serde::Serialize;
use uuid::Uuid;

use super::Game;
use super::card::CardInstance;
use super::types::Location;
use crate::{
    collection::Faction,
    error::{Error, Result},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeroInfo {
    pub id: usize,
    pub name: String,
    pub hp: usize,
    pub faction: Faction,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnemyInfo {
    pub secret_card: bool,
    pub field: HashMap<usize, CardInstance>,
    pub max_mana: usize,
    pub current_mana: usize,
    pub hand: usize,
    pub hero: HeroInfo,
    pub deck_size: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo {
    pub secret_card: Option<CardInstance>,
    pub field: HashMap<usize, CardInstance>,
    pub max_mana: usize,
    pub current_mana: usize,
    pub move_count: usize,
    pub max_move: usize,
    pub hand: Vec<CardInstance>,
    pub hero: HeroInfo,
    pub deck_size: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicGameState {
    pub game_id: Uuid,
    pub player_id: usize,
    pub turn: usize,
    pub enemy: EnemyInfo,
    pub player: PlayerInfo,
    pub winner_id: Option<usize>,
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
        let enemy_entities = game_state
            .entities
            .values()
            .filter(|e| e.owner == game_state.player_id_b);

        let mut player_field = HashMap::new();
        for entity in player_entities.clone() {
            match entity.location {
                Location::Field(pos) => {
                    player_field.insert(pos, entity.clone());
                }
                _ => (),
            }
        }

        let mut enemy_field = HashMap::new();
        for entity in enemy_entities.clone() {
            match entity.location {
                Location::Field(pos) => {
                    enemy_field.insert(pos, entity.clone());
                }
                _ => (),
            }
        }

        let enemy_hand_size = enemy_entities
            .clone()
            .filter(|e| matches!(e.location, Location::Hand))
            .count();
        let player_hand = player_entities
            .clone()
            .filter(|e| matches!(e.location, Location::Hand))
            .cloned()
            .collect();
        let enemy_deck_size = enemy_entities
            .filter(|e| matches!(e.location, Location::Deck))
            .count();
        let player_deck_size = player_entities
            .filter(|e| matches!(e.location, Location::Deck))
            .count();

        Ok(Self {
            game_id: game_state.game_id,
            player_id: game_state.player_id_a, // Will be problematic later for multiplayer
            enemy: EnemyInfo {
                secret_card: false,
                field: enemy_field,
                max_mana: player_b.base_mana,
                current_mana: player_b.mana,
                hand: enemy_hand_size,
                hero: HeroInfo {
                    id: player_b.player_id,
                    name: "Enemy IA".into(),
                    hp: player_b.hp,
                    faction: player_b.faction,
                },
                deck_size: enemy_deck_size,
            },
            player: PlayerInfo {
                secret_card: None,
                field: player_field,
                max_mana: player_a.base_mana,
                current_mana: player_a.mana,
                hand: player_hand,
                hero: HeroInfo {
                    id: player_a.player_id,
                    name: "Player".into(),
                    hp: player_a.hp,
                    faction: player_a.faction,
                },
                max_move: player_a.max_move,
                move_count: player_a.move_count,
                deck_size: player_deck_size,
            },
            winner_id: game_state.winner_id,
            turn: game_state.turn,
        })
    }
}
