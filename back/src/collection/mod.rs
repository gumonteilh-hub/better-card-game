use std::vec;

use crate::{
    game::{
        card::{CardTemplate, Keyword},
        effects::TriggeredEffect,
        types::TemplateId,
    },
    template::{PlayerTemplateTarget, TemplateEffect, TemplateTarget},
};

pub use common::get_ia_deck;
use serde::{Deserialize, Serialize};

mod common;
mod demon;
mod dragon;
mod human;

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

struct CardTemplateBuilder {
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
    triggered_effects: Vec<TriggeredEffect>,
    faction: Faction,
}
impl CardTemplateBuilder {
    fn new(
        id: TemplateId,
        cost: usize,
        name: &str,
        desc: &str,
        atk: usize,
        hp: usize,
        faction: Faction,
    ) -> Self {
        CardTemplateBuilder {
            id,
            cost,
            name: name.into(),
            desc: desc.into(),
            atk,
            hp,
            keywords: vec![],
            triggered_effects: vec![],
            on_attack: vec![],
            on_play: vec![],
            on_death: vec![],
            faction,
        }
    }

    fn keywords(mut self, keywords: Vec<Keyword>) -> CardTemplateBuilder {
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

    fn triggered_effects(mut self, effects: Vec<TriggeredEffect>) -> Self {
        self.triggered_effects = effects;
        self
    }

    fn build(self) -> CardTemplate {
        CardTemplate {
            id: self.id,
            cost: self.cost,
            name: self.name,
            description: self.desc,
            attack: self.atk,
            hp: self.hp,
            keywords: self.keywords,
            on_play: self.on_play,
            on_attack: self.on_attack,
            triggered_effects: self.triggered_effects,
            on_death: self.on_death,
            faction: self.faction,
        }
    }
}

fn card(
    id: TemplateId,
    cost: usize,
    name: &str,
    desc: &str,
    atk: usize,
    hp: usize,
    faction: Faction,
) -> CardTemplateBuilder {
    CardTemplateBuilder::new(id, cost, name, desc, atk, hp, faction)
}
