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

        let opponent_id = game_state.get_opponent(&player_id)?.player_id;
        let opponent = game_state.players.get(&opponent_id).ok_or_else(|| {
            Error::Game(format!(
                "Player B with id {} not found",
                game_state.player_id_b
            ))
        })?;

        // Single pass optimization: process all entities in one iteration
        let mut hero_field = HashMap::new();
        let mut enemy_field = HashMap::new();
        let mut player_hand = Vec::new();
        let mut enemy_hand_size = 0;
        let mut player_deck_size = 0;
        let mut enemy_deck_size = 0;

        for entity in game_state.entities.values() {
            match (entity.owner == hero.player_id, &entity.location) {
                // Hero entities
                (true, Location::Field(pos)) => {
                    hero_field.insert(*pos, entity.clone());
                }
                (true, Location::Hand) => {
                    player_hand.push(entity.clone());
                }
                (true, Location::Deck) => {
                    player_deck_size += 1;
                }
                // Enemy entities
                (false, Location::Field(pos)) => {
                    enemy_field.insert(*pos, entity.clone());
                }
                (false, Location::Hand) => {
                    enemy_hand_size += 1;
                }
                (false, Location::Deck) => {
                    enemy_deck_size += 1;
                }
                // Other locations (Graveyard, etc.) - ignore
                _ => {}
            }
        }

        Ok(Self {
            game_id: game_state.game_id,
            player_id,
            enemy: EnemyInfo {
                secret_card: false,
                field: enemy_field,
                max_mana: opponent.base_mana,
                current_mana: opponent.mana,
                hand: enemy_hand_size,
                hero: HeroInfo {
                    id: opponent.player_id,
                    name: "Enemy".into(),
                    hp: opponent.hp,
                    archetype: opponent.archetype,
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
