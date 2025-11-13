use std::vec;

use crate::{
    collection::types::{
        CardTemplate, PlayTarget, PlayerTemplateTarget, TemplateEffect, TemplateId, TemplateTarget,
    },
    game::card::Keyword,
};

pub use common::get_ia_deck;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

mod common;
mod demon;
mod dragon;
mod human;
pub mod types;

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum Race {
    DRAGON,
    DEMON,
    HUMAN,
    COMMON,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum Class {
    WARRIOR,
    MAGE,
    ROGUE,
    COMMON,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum Archetype {
    Race(Race),
    Class(Class),
}

pub fn get_collection(archetype: Archetype) -> Vec<CardTemplate> {
    match archetype {
        Archetype::Race(race) => get_collection_by_race(race),
        Archetype::Class(class) => get_collection_by_class(class),
    }
}

fn get_collection_by_class(class: Class) -> Vec<CardTemplate> {
    match class {
        Class::WARRIOR => get_warrior_cards(),
        Class::MAGE => get_mage_cards(),
        Class::ROGUE => get_rogue_cards(),
        Class::COMMON => vec![],
    }
}

fn get_collection_by_race(race: Race) -> Vec<CardTemplate> {
    match race {
        Race::DRAGON => get_dragon_cards(),
        Race::DEMON => get_demon_cards(),
        Race::HUMAN => get_human_cards(),
        Race::COMMON => vec![],
    }
}

static ALL_COLLECTION: Lazy<Vec<CardTemplate>> = Lazy::new(|| {
    let mut collection = Vec::new();
    collection.extend(dragon::get_collection());
    collection.extend(demon::get_collection());
    collection.extend(human::get_collection());
    collection.extend(common::get_collection());
    collection
});

fn get_dragon_cards() -> Vec<CardTemplate> {
    ALL_COLLECTION
        .iter()
        .filter(|&c| matches!(c.race, Race::DRAGON) || matches!(c.race, Race::COMMON))
        .cloned()
        .collect()
}

fn get_human_cards() -> Vec<CardTemplate> {
    ALL_COLLECTION
        .iter()
        .filter(|&c| matches!(c.race, Race::HUMAN) || matches!(c.race, Race::COMMON))
        .cloned()
        .collect()
}

fn get_demon_cards() -> Vec<CardTemplate> {
    ALL_COLLECTION
        .iter()
        .filter(|&c| matches!(c.race, Race::DEMON) || matches!(c.race, Race::COMMON))
        .cloned()
        .collect()
}

fn get_warrior_cards() -> Vec<CardTemplate> {
    ALL_COLLECTION
        .iter()
        .filter(|&c| matches!(c.class, Class::WARRIOR) || matches!(c.class, Class::COMMON))
        .cloned()
        .collect()
}

fn get_rogue_cards() -> Vec<CardTemplate> {
    ALL_COLLECTION
        .iter()
        .filter(|&c| matches!(c.class, Class::ROGUE) || matches!(c.class, Class::COMMON))
        .cloned()
        .collect()
}

fn get_mage_cards() -> Vec<CardTemplate> {
    ALL_COLLECTION
        .iter()
        .filter(|&c| matches!(c.class, Class::MAGE) || matches!(c.class, Class::COMMON))
        .cloned()
        .collect()
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
    race: Race,
    class: Class,
    play_target: Option<PlayTarget>,
}
impl MonsterTemplateBuilder {
    fn new(
        id: TemplateId,
        cost: usize,
        name: &str,
        desc: &str,
        atk: usize,
        hp: usize,
        race: Race,
        class: Class,
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
            play_target: None,
            race,
            class,
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

    fn on_play_with_target_choice(
        mut self,
        effects: Vec<TemplateEffect>,
        target: PlayTarget,
    ) -> Self {
        self.on_play = effects;
        self.play_target = Some(target);
        self
    }

    fn build(self) -> CardTemplate {
        CardTemplate {
            id: self.id,
            cost: self.cost,
            name: self.name,
            description: self.desc,
            race: self.race,
            class: self.class,
            play_target: self.play_target,
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
    race: Race,
    class: Class,
) -> MonsterTemplateBuilder {
    MonsterTemplateBuilder::new(id, cost, name, desc, atk, hp, race, class)
}

struct SpellTemplateBuilder {
    id: TemplateId,
    cost: usize,
    name: String,
    desc: String,
    race: Race,
    class: Class,
    effect: Vec<TemplateEffect>,
    play_target: Option<PlayTarget>,
}
impl SpellTemplateBuilder {
    fn new(id: TemplateId, cost: usize, name: &str, desc: &str, race: Race, class: Class) -> Self {
        SpellTemplateBuilder {
            id,
            cost,
            name: name.into(),
            desc: desc.into(),
            class,
            race,
            effect: vec![],
            play_target: None,
        }
    }

    fn effect(mut self, effects: Vec<TemplateEffect>) -> Self {
        self.effect = effects;
        self
    }

    fn effect_with_target_choice(
        mut self,
        effects: Vec<TemplateEffect>,
        target: PlayTarget,
    ) -> Self {
        self.effect = effects;
        self.play_target = Some(target);
        self
    }

    fn build(self) -> CardTemplate {
        CardTemplate {
            id: self.id,
            cost: self.cost,
            name: self.name,
            description: self.desc,
            race: self.race,
            class: self.class,
            play_target: self.play_target,
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
    race: Race,
    class: Class,
) -> SpellTemplateBuilder {
    SpellTemplateBuilder::new(id, cost, name, desc, race, class)
}
