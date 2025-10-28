use serde::{Deserialize, Serialize};

use crate::{
    Faction,
    game::{
        card::Keyword,
        effects::{Effect, PlayerTarget, Target},
        types::InstanceId,
    },
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
    pub faction: Faction,
    pub card_type: CardTypeTemplate,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum CardTypeTemplate {
    Monster(MonsterTemplate),
    Spell(SpellTemplate),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonsterTemplate {
    pub attack: usize,
    pub hp: usize,
    pub keywords: Vec<Keyword>,
    #[serde(default)]
    pub on_play: Vec<TemplateEffect>,
    #[serde(default)]
    pub on_attack: Vec<TemplateEffect>,
    #[serde(default)]
    pub on_death: Vec<TemplateEffect>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TemplateTarget {
    EnnemyPlayer,
    Player,
    BothPlayers,
    ItSelf,
    Allies,
    Ennemies,
    AllMonsters,
    All,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlayerTemplateTarget {
    EnnemyPlayer,
    Player,
    BothPlayers,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}

fn convert_template_target(target: &TemplateTarget) -> Target {
    match target {
        TemplateTarget::EnnemyPlayer => Target::EnnemyPlayer,
        TemplateTarget::Player => Target::Player,
        TemplateTarget::BothPlayers => Target::BothPlayers,
        TemplateTarget::ItSelf => Target::ItSelf,
        TemplateTarget::Allies => Target::Allies,
        TemplateTarget::Ennemies => Target::Ennemies,
        TemplateTarget::AllMonsters => Target::AllMonsters,
        TemplateTarget::All => Target::All,
    }
}

pub fn convert_to_effect(teff: &TemplateEffect, initiator_id: InstanceId) -> Effect {
    match teff {
        TemplateEffect::MakeDraw { player, amount } => Effect::MakeDraw {
            initiator: initiator_id,
            player: match player {
                PlayerTemplateTarget::EnnemyPlayer => PlayerTarget::EnnemyPlayer,
                PlayerTemplateTarget::Player => PlayerTarget::Player,
                PlayerTemplateTarget::BothPlayers => PlayerTarget::BothPlayers,
            },
            amount: *amount,
        },
        TemplateEffect::Heal { target, amount } => Effect::Heal {
            initiator: initiator_id,
            target: convert_template_target(target),
            amount: *amount,
        },
        TemplateEffect::Destroy { target } => Effect::Destroy {
            initiator: initiator_id,
            target: convert_template_target(target),
        },
        TemplateEffect::DealDamage { target, amount } => Effect::DealDamage {
            initiator: initiator_id,
            target: convert_template_target(target),
            amount: *amount,
        },
        TemplateEffect::Attack { target } => Effect::Attack {
            initiator: initiator_id,
            target: convert_template_target(target),
        },
        TemplateEffect::Boost { target, attack, hp } => Effect::Boost {
            initiator: initiator_id,
            target: convert_template_target(target),
            attack: *attack,
            hp: *hp,
        },
    }
}
