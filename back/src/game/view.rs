use std::collections::HashMap;

use serde::Serialize;
use uuid::Uuid;

use super::Game;
use super::types::Location;
use crate::{
    collection::Archetype,
    error::{Error, Result},
    game::{card::CardInstance, types::PlayerId},
};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeroInfo {
    pub id: usize,
    pub name: String,
    pub hp: usize,
    pub archetype: Archetype,
}

#[derive(Serialize, Debug, Clone)]
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

#[derive(Serialize, Debug, Clone)]
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

#[derive(Serialize, Debug, Clone)]
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
    // todo make use of player_id
    pub fn new(game_state: &Game, player_id: PlayerId) -> Result<Self> {
        let hero = game_state.players.get(&player_id).ok_or_else(|| {
            Error::Game(format!(
                "Player with id {} not found",
                game_state.player_id_a
            ))
        })?;

        let oponent_id = game_state.get_opponent(&player_id)?.player_id;
        let oponent = game_state.players.get(&oponent_id).ok_or_else(|| {
            Error::Game(format!(
                "Player B with id {} not found",
                game_state.player_id_b
            ))
        })?;

        let hero_entities = game_state
            .entities
            .values()
            .filter(|e| e.owner == hero.player_id);
        let enemy_entities = game_state
            .entities
            .values()
            .filter(|e| e.owner == oponent.player_id);

        let mut hero_field = HashMap::new();
        for entity in hero_entities.clone() {
            if let Location::Field(pos) = entity.location {
                hero_field.insert(pos, entity.clone());
            }
        }

        let mut enemy_field = HashMap::new();
        for entity in enemy_entities.clone() {
            if let Location::Field(pos) = entity.location {
                enemy_field.insert(pos, entity.clone());
            }
        }

        let enemy_hand_size = enemy_entities
            .clone()
            .filter(|e| matches!(e.location, Location::Hand))
            .count();
        let player_hand = hero_entities
            .clone()
            .filter(|e| matches!(e.location, Location::Hand))
            .cloned()
            .collect();
        let enemy_deck_size = enemy_entities
            .filter(|e| matches!(e.location, Location::Deck))
            .count();
        let player_deck_size = hero_entities
            .filter(|e| matches!(e.location, Location::Deck))
            .count();

        Ok(Self {
            game_id: game_state.game_id,
            player_id,
            enemy: EnemyInfo {
                secret_card: false,
                field: enemy_field,
                max_mana: oponent.base_mana,
                current_mana: oponent.mana,
                hand: enemy_hand_size,
                hero: HeroInfo {
                    id: oponent.player_id,
                    name: "Enemy".into(),
                    hp: oponent.hp,
                    archetype: oponent.archetype,
                },
                deck_size: enemy_deck_size,
            },
            player: PlayerInfo {
                secret_card: None,
                field: hero_field,
                max_mana: hero.base_mana,
                current_mana: hero.mana,
                hand: player_hand,
                hero: HeroInfo {
                    id: hero.player_id,
                    name: "Player".into(),
                    hp: hero.hp,
                    archetype: hero.archetype,
                },
                max_move: hero.max_move,
                move_count: hero.move_count,
                deck_size: player_deck_size,
            },
            winner_id: game_state.winner_id,
            turn: game_state.turn,
        })
    }
}
