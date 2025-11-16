use serde::{Deserialize, Serialize};

use crate::{
    Race,
    collection::Class,
    game::{card::Keyword, types::PlayerId},
};

pub type TemplateId = usize;

#[derive(Debug, Clone, Serialize)]
pub struct SpellTemplate {
    pub effect: Vec<TemplateEffect>,
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
    pub on_alone: Vec<TemplateEffect>,
    pub on_surrounded: Vec<TemplateEffect>,
    pub on_play: Vec<TemplateEffect>,
    pub on_attack: Vec<TemplateEffect>,
    pub on_death: Vec<TemplateEffect>,
}

#[derive(Debug, Serialize, Clone)]
pub enum TemplateTarget {
    EnnemyPlayer,
    Player,
    BothPlayers,
    ItSelf,
    Allies,
    Ennemies,
    AllMonsters,
    All,
    Choose,
    Matching(TargetMatcherTemplate),
    And(Box<TemplateTarget>, Box<TemplateTarget>),
    Or(Box<TemplateTarget>, Box<TemplateTarget>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlayerTemplateTarget {
    EnnemyPlayer,
    Player,
    BothPlayers,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum TemplateEffect {
    Boost {
        target: TemplateTarget,
        attack: usize,
        hp: usize,
    },
    MakeDraw {
        player: PlayerTemplateTarget,
        amount: usize,
    },
    Heal {
        target: TemplateTarget,
        amount: usize,
    },
    Destroy {
        target: TemplateTarget,
    },
    DealDamage {
        target: TemplateTarget,
        amount: usize,
    },
    // Custom(String),
    Attack {
        target: TemplateTarget,
    },
    Summon {
        side: PlayerTemplateTarget,
        target: CardTemplate,
    },
    IncreaseMaxMana {
        player: PlayerTemplateTarget,
        amount: usize,
    },
    RefreshMana {
        player: PlayerTemplateTarget,
        amount: usize,
    },
    SetAbsoluteMaxMana {
        player: PlayerTemplateTarget,
        amount: usize,
    },
}
