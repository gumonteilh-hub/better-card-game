pub mod action;
pub mod card;
pub mod effects;
pub mod events;
pub mod logic;
pub mod player;
pub mod types;
pub mod user_actions;
mod utils;
pub mod view;

#[cfg(test)]
mod tests;

use std::collections::{HashMap, VecDeque};

use crate::UserDeck;
use crate::collection::types::CardTemplate;
use crate::error::{Error, Result};
use crate::game::action::Action;
use crate::game::card::CardInstance;
use crate::game::effects::Effect;
use crate::game::logic::execute_effect;
use crate::game::types::Location;

use self::events::EventManager;
use self::player::PlayerInstance;
use self::types::{InstanceId, PlayerId};

#[derive(Debug, Clone)]
pub struct Game {
    pub game_id: uuid::Uuid,
    pub player_id_a: usize,
    pub player_id_b: usize,
    pub entities: HashMap<InstanceId, CardInstance>,
    pub effect_queue: VecDeque<Effect>,
    pub players: HashMap<PlayerId, PlayerInstance>,
    pub turn: usize,
    pub current_player: PlayerId,
    pub event_manager: EventManager,
    pub winner_id: Option<PlayerId>,
    pub vs_ia: bool,
}

impl Game {
    pub fn new(
        deck_a: UserDeck,
        deck_b: UserDeck,
        collection_a: Vec<CardTemplate>,
        collection_b: Vec<CardTemplate>,
        vs_ia: bool,
    ) -> Result<Self> {
        let mut entity_id = 0;

        let mut players = HashMap::new();
        let player_id_a = entity_id;
        players.insert(
            player_id_a,
            PlayerInstance::new(player_id_a, 1, deck_a.archetype),
        );
        entity_id += 1;
        let player_id_b = entity_id;
        players.insert(
            player_id_b,
            PlayerInstance::new(player_id_b, 0, deck_b.archetype),
        );
        entity_id += 1;

        let mut entities = HashMap::new();
        for card in deck_a.cards.iter() {
            let template = collection_a
                .iter()
                .find(|t| t.id == *card)
                .ok_or_else(|| Error::Game(format!("Template with id {} not found", card)))?;
            entities.insert(
                entity_id,
                CardInstance::new(entity_id, player_id_a, template, player_id_b),
            );
            entity_id += 1;
        }
        for card in deck_b.cards.iter() {
            let template = collection_b
                .iter()
                .find(|t| t.id == *card)
                .ok_or_else(|| Error::Game(format!("Template with id {} not found", card)))?;
            entities.insert(
                entity_id,
                CardInstance::new(entity_id, player_id_b, template, player_id_a),
            );
            entity_id += 1;
        }

        let mut queue = VecDeque::new();
        queue.push_back(Effect::AutoDraw {
            player: player_id_a,
            amount: 5,
        });
        queue.push_back(Effect::AutoDraw {
            player: player_id_b,
            amount: 5,
        });

        Ok(Self {
            game_id: uuid::Uuid::new_v4(),
            player_id_a,
            player_id_b,
            effect_queue: queue,
            players,
            entities,
            turn: 1,
            current_player: player_id_a,
            event_manager: EventManager::new(),
            winner_id: None,
            vs_ia,
        })
    }

    // compute all the effects in the effect_queue of the game
    // even the generated effects by other effects are compute
    // after the method is called, expect the game_state being up to date and the effect queue to
    // be empty
    pub fn compute_commands(&mut self) -> Result<Vec<Action>> {
        let mut all_actions = Vec::new();
        while let Some(effect) = self.effect_queue.pop_front() {
            let mut performed_actions = execute_effect(&effect, self)?;
            all_actions.append(&mut performed_actions);
        }

        Ok(all_actions)
    }

    pub fn get_opponent(&self, player_id: &PlayerId) -> Result<&PlayerInstance> {
        let oponent = self
            .players
            .iter()
            .find(|(id, instance)| *id != player_id)
            .ok_or_else(|| Error::Game("This monster has already attacked this turn".into()))?;

        Ok(oponent.1)
    }

    pub fn get_mut_player(&mut self, player_id: PlayerId) -> Result<&mut PlayerInstance> {
        self.players
            .get_mut(&player_id)
            .ok_or_else(|| Error::Game(format!("Player with id {} not found", player_id)))
    }

    pub fn get_player(&mut self, player_id: PlayerId) -> Result<&PlayerInstance> {
        self.players
            .get(&player_id)
            .ok_or_else(|| Error::Game(format!("Player with id {} not found", player_id)))
    }

    pub fn get_entity(&self, entity_id: InstanceId) -> Result<&CardInstance> {
        let entity = self
            .entities
            .get(&entity_id)
            .ok_or_else(|| Error::Game(format!("Card with id {} not found", entity_id)))?;
        Ok(entity)
    }

    pub fn get_mut_entity(&mut self, entity_id: InstanceId) -> Result<&mut CardInstance> {
        let entity = self
            .entities
            .get_mut(&entity_id)
            .ok_or_else(|| Error::Game(format!("Card with id {} not found", entity_id)))?;
        Ok(entity)
    }

    /// return the monsters on the field of the player with player_id
    /// as a map were the key is the position of the field and the value an immutable reference ok
    /// the monster
    pub fn get_field_with_position(&self, player_id: PlayerId) -> HashMap<usize, &CardInstance> {
        let mut result: HashMap<usize, &CardInstance> = HashMap::new();

        self.entities
            .iter()
            .filter(|(_, e)| e.owner == player_id && matches!(e.location, Location::Field(_)))
            .for_each(|(_, c)| {
                match c.location {
                    Location::Field(pos) => result.insert(pos, c),
                    _ => todo!(),
                };
            });

        result
    }

    pub fn get_field(&self, player_id: PlayerId) -> HashMap<&InstanceId, &CardInstance> {
        self.entities
            .iter()
            .filter(|(_, e)| e.owner == player_id && matches!(e.location, Location::Field(_)))
            .collect()
    }

    pub fn get_mut_field(
        &mut self,
        player_id: PlayerId,
    ) -> HashMap<&InstanceId, &mut CardInstance> {
        self.entities
            .iter_mut()
            .filter(|(_, e)| e.owner == player_id && matches!(e.location, Location::Field(_)))
            .collect()
    }

    pub fn get_hand(&self, player_id: PlayerId) -> HashMap<&InstanceId, &CardInstance> {
        self.entities
            .iter()
            .filter(|(_, e)| e.owner == player_id && e.location == Location::Hand)
            .collect()
    }
}
