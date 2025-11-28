use serde::Serialize;

use crate::{
    Race,
    collection::Class,
    game::{
        card::Keyword,
        effects::{Effect, PlayerTarget},
        types::PlayerId,
    },
};

pub type TemplateId = usize;

#[derive(Debug, Clone, Serialize)]
pub struct SpellTemplate {
    pub effect: Vec<Effect>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CardTemplate {
    pub id: TemplateId,
    pub cost: usize,
    pub name: String,
    pub description: String,
    pub race: Race,
    pub class: Class,
    pub card_type: CardTypeTemplate,
    pub play_target: Option<PlayTargetTemplate>,
}

#[derive(Debug, Clone, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub struct PlayTargetTemplate {
    pub strict: bool,
    pub amount: usize,
    pub matcher: TargetMatcherTemplate,
}

#[derive(Debug, Clone, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub struct PlayTarget {
    pub strict: bool,
    pub amount: usize,
    pub matcher: TargetMatcher,
}

#[derive(Debug, Clone, Serialize, Copy)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum TargetMatcher {
    Race(Race),
    Class(Class),
    Owner(PlayerId),
}

#[derive(Debug, Clone, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Side {
    Player,
    Enemy,
}

#[derive(Debug, Clone, Serialize, Copy)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum TargetMatcherTemplate {
    Race(Race),
    Class(Class),
    Side(Side),
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum CardTypeTemplate {
    Monster(MonsterTemplate),
    Spell(SpellTemplate),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonsterTemplate {
    pub attack: usize,
    pub hp: usize,
    pub keywords: Vec<Keyword>,
    pub on_alone: Vec<Effect>,
    pub on_surrounded: Vec<Effect>,
    pub on_play: Vec<Effect>,
    pub on_attack: Vec<Effect>,
    pub on_defend: Vec<Effect>,
    pub on_damaged: Vec<Effect>,
    pub on_kill: Vec<Effect>,
    pub on_turn_end: Vec<Effect>,
    pub on_turn_start: Vec<Effect>,
    pub on_death: Vec<Effect>,
}

#[derive(Debug, Serialize, Clone)]
pub enum Comparator {
    More,
    Less,
    Equal,
}

#[derive(Debug, Serialize, Clone)]
pub enum ComparableVariable {
    Mana {
        player: PlayerTarget,
        comparator: Comparator,
        value: usize,
    },
    Hp {
        player: PlayerTarget,
        comparator: Comparator,
        value: usize,
    },
    Turn {
        comparator: Comparator,
        value: usize,
    },
    MaxMana {
        player: PlayerTarget,
        comparator: Comparator,
        value: usize,
    },
    FieldLength {
        player: PlayerTarget,
        comparator: Comparator,
        value: usize,
    },
    OwnMonster {
        player: PlayerTarget,
        comparator: Comparator,
        value: usize,
        matching: TargetMatcherTemplate,
    },
}
