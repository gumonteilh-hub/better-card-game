use serde::{Deserialize, Serialize};

use crate::game::{
    card::CardInstance,
    effects::{Effect, PlayerTarget, Target},
};

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

pub fn convert_to_effect(teff: &TemplateEffect, initiator: &CardInstance) -> Effect {
    match teff {
        TemplateEffect::MakeDraw { player, amount } => Effect::MakeDraw {
            initiator: initiator.id,
            player: match player {
                PlayerTemplateTarget::EnnemyPlayer => PlayerTarget::EnnemyPlayer,
                PlayerTemplateTarget::Player => PlayerTarget::Player,
                PlayerTemplateTarget::BothPlayers => PlayerTarget::BothPlayers,
            },
            amount: *amount,
        },
        TemplateEffect::Heal { target, amount } => Effect::Heal {
            initiator: initiator.id,
            target: convert_template_target(target),
            amount: *amount,
        },
        TemplateEffect::Destroy { target } => Effect::Destroy {
            initiator: initiator.id,
            target: convert_template_target(target),
        },
        TemplateEffect::DealDamage { target, amount } => Effect::DealDamage {
            initiator: initiator.id,
            target: convert_template_target(target),
            amount: *amount,
        },
        TemplateEffect::Attack { target } => Effect::Attack {
            initiator: initiator.id,
            target: convert_template_target(target),
        },
        TemplateEffect::Boost { target, attack, hp } => Effect::Boost {
            initiator: initiator.id,
            target: convert_template_target(target),
            attack: *attack,
            hp: *hp,
        },
    }
}
