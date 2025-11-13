pub mod action;
pub mod card;
pub mod effects;
pub mod events;
pub mod logic;
pub mod player;
pub mod types;
mod user_actions;
mod utils;
pub mod view;

#[cfg(test)]
mod tests;

use std::collections::{HashMap, VecDeque};

use crate::collection::types::CardTemplate;
use crate::error::{Error, Result};
use crate::game::action::Action;
use crate::game::card::{CardInstance, Keyword};
use crate::game::effects::{Effect, Target};
use crate::game::logic::execute_effect;
use crate::game::types::Location;
use crate::{UserDeck, ia};

use self::events::EventManager;
use self::player::PlayerInstance;
use self::types::{InstanceId, PlayerId};

pub const DEFENSE_POSITIONS: [usize; 5] = [1, 2, 4, 5, 7];
pub const ATTACK_POSITIONS: [usize; 5] = [0, 2, 3, 5, 6];

fn get_linked_positions(position: usize) -> Result<Vec<usize>> {
    match position {
        0 => Ok(vec![1, 2]),
        1 => Ok(vec![0, 2]),
        2 => Ok(vec![0, 1, 3, 4]),
        3 => Ok(vec![2, 4, 5]),
        4 => Ok(vec![2, 3, 5]),
        5 => Ok(vec![3, 4, 6, 7]),
        6 => Ok(vec![5, 7]),
        7 => Ok(vec![5, 6]),
        _ => Err(Error::Game("Invalid starting position".into())),
    }
}

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
                CardInstance::new(entity_id, player_id_a, template),
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
                CardInstance::new(entity_id, player_id_b, template),
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

    pub fn move_card(
        &mut self,
        player: PlayerId,
        card_id: InstanceId,
        position: usize,
    ) -> Result<()> {
        let card = self
            .entities
            .get(&card_id)
            .ok_or_else(|| Error::Game(format!("Card with id {} not found", card_id)))?;

        if card.owner != player {
            return Err(Error::Game("You can only move your monsters".into()));
        }

        let starting_position = match card.location {
            Location::Field(pos) => pos,
            _ => return Err(Error::Game("Card must be on the field".into())),
        };

        if !get_linked_positions(starting_position)?.contains(&position) {
            return Err(Error::Game("Target position is not valid".into()));
        }

        if self
            .get_field_with_position(card.owner)
            .contains_key(&position)
        {
            return Err(Error::Game("You can't move to a position not empty".into()));
        }

        if self.get_player(self.current_player)?.move_count == 0 {
            return Err(Error::Game("You don't have any move left".into()));
        }

        let owner = self.get_mut_player(self.current_player)?;
        owner.move_count -= 1;

        let card = self
            .entities
            .get_mut(&card_id)
            .ok_or_else(|| Error::Game(format!("Card with id {} not found", card_id)))?;

        card.location = Location::Field(position);

        Ok(())
    }

    pub fn play_spell(
        &mut self,
        owner: PlayerId,
        card_id: usize,
        selected_targets: Option<Vec<InstanceId>>,
    ) -> Result<()> {
        user_actions::play_spell::play_spell(self, owner, card_id, selected_targets)?;
        Ok(())
    }

    pub fn play_monster(
        &mut self,
        owner: PlayerId,
        card_id: InstanceId,
        position: usize,
        selected_targets: Option<Vec<InstanceId>>,
    ) -> Result<Vec<Action>> {
        let actions = user_actions::play_monster::play_monster(
            self,
            owner,
            card_id,
            position,
            selected_targets,
        )?;
        Ok(actions)
    }

    pub fn end_turn(&mut self, ending_player: PlayerId) -> Result<Vec<Action>> {
        let mut actions = Vec::new();
        let starting_player = *self.players.keys().find(|p| **p != ending_player).unwrap();

        actions.push(Action::StartTurn(starting_player));
        self.current_player = starting_player;
        self.effect_queue.push_back(Effect::AutoDraw {
            player: starting_player,
            amount: 1,
        });

        let current_player_instance = self.get_mut_player(starting_player)?;

        if current_player_instance.base_mana < 10 {
            self.effect_queue.push_back(Effect::IncreaseMaxMana {
                initiator: starting_player,
                player: effects::PlayerTarget::Player,
                amount: 1,
            });
        }

        let base_mana = self.get_player(self.current_player)?.base_mana;
        self.effect_queue.push_back(Effect::RefreshMana {
            initiator: starting_player,
            player: effects::PlayerTarget::Player,
            amount: base_mana + 1,
        });

        self.get_mut_player(starting_player)?.move_count = 3;

        for (_, monster) in self.get_mut_field(starting_player) {
            match &mut monster.card_type {
                card::CardTypeInstance::Monster(monster_instance) => {
                    monster_instance.attack_count = 0;
                    monster_instance.asleep = false;
                }
                card::CardTypeInstance::Spell(spell_instance) => {
                    return Err(Error::Game("There shouldn't be spell on the field".into()));
                }
            }
        }

        let mut reset_turn_actions = self.compute_commands()?;
        actions.append(&mut reset_turn_actions);

        if starting_player == self.player_id_b && self.vs_ia {
            let mut ia_actions = ia::ai_play_turn(self, self.player_id_b)?;
            actions.append(&mut ia_actions);
        }
        Ok(actions)
    }

    // Pure logic, no checks, checks should be done before pushing a command
    pub fn compute_commands(&mut self) -> Result<Vec<Action>> {
        let mut all_actions = Vec::new();
        while let Some(effect) = self.effect_queue.pop_front() {
            let mut performed_actions = execute_effect(&effect, self)?;
            all_actions.append(&mut performed_actions);
        }

        Ok(all_actions)
    }

    pub fn attack(
        &mut self,
        player: PlayerId,
        initiator_id: InstanceId,
        target_id: InstanceId,
    ) -> Result<()> {
        let initiator = self
            .entities
            .get(&initiator_id)
            .ok_or_else(|| Error::Game(format!("Attacker with id {} not found", initiator_id)))?;

        if target_id == 0 || target_id == 1 {
            if initiator.owner == target_id {
                return Err(Error::Game("You can't attack your own player".into()));
            }
            if self
                .get_field_with_position(target_id)
                .iter()
                .any(|(pos, _)| DEFENSE_POSITIONS.contains(pos))
            {
                return Err(Error::Game(
                    "You can't attack the enemy player if he has a monster in defense".into(),
                ));
            }
        } else {
            let target = self
                .entities
                .get(&target_id)
                .ok_or_else(|| Error::Game(format!("Target with id {} not found", target_id)))?;

            if initiator.owner == target.owner {
                return Err(Error::Game("You can't attack your own monster".into()));
            }
        }
        match initiator.location {
            Location::Field(pos) => {
                if !ATTACK_POSITIONS.contains(&pos) {
                    return Err(Error::Game(
                        "This monster must be on an attack slot to attack".into(),
                    ));
                }
            }
            _ => {
                return Err(Error::Game(
                    "This monster must be on the field to attack".into(),
                ));
            }
        };
        let initiator = self
            .entities
            .get_mut(&initiator_id)
            .ok_or_else(|| Error::Game(format!("Attacker with id {} not found", initiator_id)))?;

        match &mut initiator.card_type {
            card::CardTypeInstance::Monster(monster_instance) => {
                if monster_instance.asleep {
                    return Err(Error::Game(
                        "This monster can't attack on his first turn".into(),
                    ));
                }

                if monster_instance.keywords.contains(&Keyword::Windfury) {
                    if monster_instance.attack_count > 1 {
                        return Err(Error::Game(
                            "This monster has already attacked this turn".into(),
                        ));
                    }
                } else if monster_instance.attack_count > 0 {
                    return Err(Error::Game(
                        "This monster has already attacked this turn".into(),
                    ));
                }

                self.effect_queue.push_back(Effect::Attack {
                    initiator: initiator_id,
                    target: Target::Id(target_id),
                });
                monster_instance.attack_count += 1;

                Ok(())
            }
            card::CardTypeInstance::Spell(spell_instance) => {
                Err(Error::Game("A spell can not attack".into()))
            }
        }
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
