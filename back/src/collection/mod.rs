use std::vec;

use crate::{
    collection::types::{
        CardTemplate, PlayerTemplateTarget, TemplateEffect, TemplateId, TemplateTarget,
    },
    game::card::Keyword,
};

pub use common::get_ia_deck;
use serde::{Deserialize, Serialize};

mod common;
mod demon;
mod dragon;
mod human;
pub mod types;

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum Faction {
    DRAGON,
    DEMON,
    HUMAN,
    COMMON,
}

pub fn get_collection(faction: Faction) -> Vec<CardTemplate> {
    let mut all_cards = Vec::new();

    match faction {
        Faction::DRAGON => all_cards.extend(dragon::get_collection()),
        Faction::DEMON => all_cards.extend(demon::get_collection()),
        Faction::HUMAN => all_cards.extend(human::get_collection()),
        Faction::COMMON => (),
    }
    all_cards.extend(common::get_collection());

    all_cards
}

pub fn draw(player: PlayerTemplateTarget, amount: usize) -> TemplateEffect {
    TemplateEffect::MakeDraw { player, amount }
}

pub fn deal_damage(target: TemplateTarget, amount: usize) -> TemplateEffect {
    TemplateEffect::DealDamage { target, amount }
}

pub fn heal(target: TemplateTarget, amount: usize) -> TemplateEffect {
    TemplateEffect::Heal { target, amount }
}

pub fn boost(target: TemplateTarget, attack: usize, hp: usize) -> TemplateEffect {
    TemplateEffect::Boost { target, attack, hp }
}

struct MonsterTemplateBuilder {
    id: TemplateId,
    cost: usize,
    name: String,
    desc: String,
    atk: usize,
    hp: usize,
    keywords: Vec<Keyword>,
    on_play: Vec<TemplateEffect>,
    on_attack: Vec<TemplateEffect>,
    on_death: Vec<TemplateEffect>,
    faction: Faction,
}
impl MonsterTemplateBuilder {
    fn new(
        id: TemplateId,
        cost: usize,
        name: &str,
        desc: &str,
        atk: usize,
        hp: usize,
        faction: Faction,
    ) -> Self {
        MonsterTemplateBuilder {
            id,
            cost,
            name: name.into(),
            desc: desc.into(),
            atk,
            hp,
            keywords: vec![],
            on_attack: vec![],
            on_play: vec![],
            on_death: vec![],
            faction,
        }
    }

    fn keywords(mut self, keywords: Vec<Keyword>) -> MonsterTemplateBuilder {
        self.keywords = keywords;
        self
    }

    fn on_play(mut self, effects: Vec<TemplateEffect>) -> Self {
        self.on_play = effects;
        self
    }

    fn on_attack(mut self, effects: Vec<TemplateEffect>) -> Self {
        self.on_attack = effects;
        self
    }

    fn on_death(mut self, effects: Vec<TemplateEffect>) -> Self {
        self.on_death = effects;
        self
    }

    fn build(self) -> CardTemplate {
        CardTemplate {
            id: self.id,
            cost: self.cost,
            name: self.name,
            description: self.desc,
            faction: self.faction,
            card_type: types::CardTypeTemplate::Monster(types::MonsterTemplate {
                attack: self.atk,
                hp: self.hp,
                keywords: self.keywords,
                on_play: self.on_play,
                on_attack: self.on_attack,
                on_death: self.on_death,
            }),
        }
    }
}

fn monster(
    id: TemplateId,
    cost: usize,
    name: &str,
    desc: &str,
    atk: usize,
    hp: usize,
    faction: Faction,
) -> MonsterTemplateBuilder {
    MonsterTemplateBuilder::new(id, cost, name, desc, atk, hp, faction)
}

struct SpellTemplateBuilder {
    id: TemplateId,
    cost: usize,
    name: String,
    desc: String,
    faction: Faction,
    effect: Vec<TemplateEffect>,
}
impl SpellTemplateBuilder {
    fn new(id: TemplateId, cost: usize, name: &str, desc: &str, faction: Faction) -> Self {
        SpellTemplateBuilder {
            id,
            cost,
            name: name.into(),
            desc: desc.into(),
            faction,
            effect: vec![],
        }
    }

    fn effect(mut self, effects: Vec<TemplateEffect>) -> Self {
        self.effect = effects;
        self
    }

    fn build(self) -> CardTemplate {
        CardTemplate {
            id: self.id,
            cost: self.cost,
            name: self.name,
            description: self.desc,
            faction: self.faction,
            card_type: types::CardTypeTemplate::Spell(types::SpellTemplate {
                effect: self.effect,
            }),
        }
    }
}

fn spell(
    id: TemplateId,
    cost: usize,
    name: &str,
    desc: &str,
    faction: Faction,
) -> SpellTemplateBuilder {
    SpellTemplateBuilder::new(id, cost, name, desc, faction)
}
