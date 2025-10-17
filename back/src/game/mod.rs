pub mod action;
pub mod card;
pub mod effects;
pub mod events;
pub mod logic;
pub mod player;
pub mod types;
pub mod view;

use std::collections::{HashMap, VecDeque};

use crate::error::{Error, Result};
use crate::game::action::Action;
use crate::game::card::Keyword;
use crate::game::effects::{Effect, Target};
use crate::game::logic::execute_effect;
use crate::game::types::Location;
use crate::{ia, UserDeck};

use self::card::{CardInstance};
use self::events::EventManager;
use self::player::PlayerInstance;
use self::types::{EntityId, PlayerId};

#[derive(Debug, Clone)]
pub struct Game {
    pub player_id_a: usize,
    pub player_id_b: usize, //IA
    pub entities: HashMap<EntityId, CardInstance>,
    pub effect_queue: VecDeque<Effect>,
    pub players: HashMap<PlayerId, PlayerInstance>,
    pub turn: usize,
    pub current_player: PlayerId,
    pub event_manager: EventManager,
    pub winner_id: Option<PlayerId>,
}



impl Game {
    pub fn new(deck_a: UserDeck, deck_b: UserDeck) -> Self {
        let mut entity_id = 0;

        let mut players = HashMap::new();
        let player_id_a = entity_id;
        players.insert(
            player_id_a,
            PlayerInstance::new(player_id_a, 1, deck_a.faction),
        );
        entity_id += 1;
        let player_id_b = entity_id;
        players.insert(
            player_id_b,
            PlayerInstance::new(player_id_b, 0, deck_b.faction),
        );
        entity_id += 1;

        let mut entities = HashMap::new();
        for card in deck_a.cards.iter() {
            entities.insert(entity_id, CardInstance::new(entity_id, player_id_a, card));
            entity_id += 1;
        }
        for card in deck_b.cards.iter() {
            entities.insert(entity_id, CardInstance::new(entity_id, player_id_b, card));
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

        Self {
            player_id_a,
            player_id_b,
            effect_queue: queue,
            players,
            entities,
            turn: 1,
            current_player: player_id_a,
            event_manager: EventManager::new(),
            winner_id: None,
        }
    }

    pub fn play_card(&mut self, card_id: EntityId, position: usize) -> Result<()> {
        let owner = self
            .entities
            .get(&card_id)
            .ok_or_else(|| Error::Game(format!("Card with id {} not found", card_id)))?
            .owner;

        if self
            .get_field(owner)
            .iter()
            .any(|(_, c)| c.location == Location::Field(position))
        {
            return Err(Error::Game(format!("This place on the field is not empty")));
        }

        if self.get_field(owner).len() >= 7 {
            return Err(Error::Game("Your board is already full".into()));
        }

        let card = self.get_entity(card_id)?;

        let card_cost = card.template.cost;

        let player = self
            .players
            .get_mut(&owner)
            .ok_or_else(|| Error::Game(format!("Player with id {} not found", owner)))?;

        if player.mana < card_cost {
            return Err(Error::Game(
                "You don't have enough mana to play this card".into(),
            ));
        }

        player.mana -= card_cost;

        self.effect_queue.push_back(Effect::SummonFromHand {
            entity_id: card_id,
            position: position,
        });
        Ok(())
    }

    pub fn next_turn(&mut self) -> Result<Vec<Action>> {
        let mut actions = Vec::new();
        if self.current_player == self.player_id_a {
            self.current_player = self.player_id_b;
        } else {
            self.current_player = self.player_id_a;
        };

        self.effect_queue.push_back(Effect::AutoDraw {
            player: self.current_player,
            amount: 1,
        });

        let current_player_instance = self.get_mut_player(self.current_player)?;

        if current_player_instance.base_mana < 10 {
            self.effect_queue.push_back(Effect::IncreaseMaxMana {
                initiator: self.current_player,
                player: effects::PlayerTarget::Player,
                amount: 1,
            });
        }

        let base_mana = self.get_mut_player(self.current_player)?.base_mana;
        self.effect_queue.push_back(Effect::RefreshMana {
            initiator: self.current_player,
            player: effects::PlayerTarget::Player,
            amount: base_mana + 1,
        });

        for (_, monster) in self.get_mut_field(self.current_player) {
            monster.attack_count = 0;
            monster.asleep = false;
        }

        let mut reset_turn_actions = self.compute_commands()?;
        actions.append(&mut reset_turn_actions);

        if self.current_player == self.player_id_b {
            actions = ia::ai_play_turn(self, self.player_id_b)?;
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

    pub fn attack(&mut self, initiator_id: EntityId, target_id: EntityId) -> Result<()> {
        let initiator = self
            .entities
            .get(&initiator_id)
            .ok_or_else(|| Error::Game(format!("Attacker with id {} not found", initiator_id)))?;

        if target_id == 0 || target_id == 1 {
            if initiator.owner == target_id {
                return Err(Error::Game("You can't attack your own player".into()));
            }
            if self
                .get_field(target_id)
                .iter()
                .filter(|c| c.1.template.keywords.contains(&card::Keyword::Taunt))
                .count()
                > 0
            {
                return Err(Error::Game(
                "You must destroy all Taunt enemy monsters first before attacking the enemy Heros".into(),
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
            if !target.template.keywords.contains(&Keyword::Taunt)
                && self
                    .get_field(target.owner)
                    .iter()
                    .filter(|c| c.1.template.keywords.contains(&card::Keyword::Taunt))
                    .count()
                    > 0
            {
                return Err(Error::Game(
                    "You must focus Taunt monsters before attacking other monsters".into(),
                ));
            }
        }
        if initiator.asleep {
            return Err(Error::Game(
                "This monster can't attack on his first turn".into(),
            ));
        }

        if initiator.template.keywords.contains(&Keyword::Windfury) {
            if initiator.attack_count > 1 {
                return Err(Error::Game(
                    "This monster has already attacked this turn".into(),
                ));
            }
        } else {
            if initiator.attack_count > 0 {
                return Err(Error::Game(
                    "This monster has already attacked this turn".into(),
                ));
            }
        }

        self.effect_queue.push_back(Effect::Attack {
            initiator: initiator.id,
            target: Target::Id(target_id),
        });

        self.entities
            .get_mut(&initiator_id)
            .ok_or_else(|| Error::Game(format!("Attacker with id {} not found", initiator_id)))?
            .attack_count += 1;

        Ok(())
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

    pub fn get_entity(&self, entity_id: EntityId) -> Result<&CardInstance> {
        let entity = self
            .entities
            .get(&entity_id)
            .ok_or_else(|| Error::Game(format!("Card with id {} not found", entity_id)))?;
        Ok(entity)
    }

    pub fn get_mut_entity(&mut self, entity_id: EntityId) -> Result<&mut CardInstance> {
        let entity = self
            .entities
            .get_mut(&entity_id)
            .ok_or_else(|| Error::Game(format!("Card with id {} not found", entity_id)))?;
        Ok(entity)
    }

    pub fn get_field(&self, player_id: PlayerId) -> HashMap<&EntityId, &CardInstance> {
        self.entities
            .iter()
            .filter(|(_, e)| e.owner == player_id && matches!(e.location, Location::Field(_)))
            .collect()
    }

    pub fn get_mut_field(&mut self, player_id: PlayerId) -> HashMap<&EntityId, &mut CardInstance> {
        self.entities
            .iter_mut()
            .filter(|(_, e)| e.owner == player_id && matches!(e.location, Location::Field(_)))
            .collect()
    }

    pub fn get_hand(&self, player_id: PlayerId) -> HashMap<&EntityId, &CardInstance> {
        self.entities
            .iter()
            .filter(|(_, e)| e.owner == player_id && e.location == Location::Hand)
            .collect()
    }
}
