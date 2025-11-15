use serde::{Deserialize, Serialize};

use crate::{
    Race,
    collection::Class,
    game::{
        card::Keyword,
        effects::{Effect, PlayerTarget, Target},
        types::{InstanceId, PlayerId},
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

impl PlayTargetTemplate {
    pub fn convert(&self, owner: PlayerId, oponent_id: PlayerId) -> PlayTarget {
        PlayTarget {
            strict: self.strict,
            amount: self.amount,
            matcher: self.matcher.convert(owner, oponent_id),
        }
    }
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
impl TargetMatcherTemplate {
    pub fn convert(&self, owner: PlayerId, oponent_id: PlayerId) -> TargetMatcher {
        match self {
            TargetMatcherTemplate::Race(race) => TargetMatcher::Race(*race),
            TargetMatcherTemplate::Class(class) => TargetMatcher::Class(*class),
            TargetMatcherTemplate::Side(side) => match side {
                Side::Player => TargetMatcher::Owner(owner),
                Side::Enemy => TargetMatcher::Owner(oponent_id),
            },
        }
    }
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
    #[serde(default)]
    pub on_play: Vec<TemplateEffect>,
    #[serde(default)]
    pub on_attack: Vec<TemplateEffect>,
    #[serde(default)]
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
        TemplateTarget::Choose => Target::Ids(vec![]),
        TemplateTarget::Matching(target_matcher) => Target::Matching(*target_matcher),
        TemplateTarget::And(a, b) => Target::And(
            Box::new(convert_template_target(a)),
            Box::new(convert_template_target(b)),
        ),
        TemplateTarget::Or(a, b) => Target::Or(
            Box::new(convert_template_target(a)),
            Box::new(convert_template_target(b)),
        ),
    }
}

fn convert_template_player_target(target: &PlayerTemplateTarget) -> PlayerTarget {
    match target {
        PlayerTemplateTarget::EnnemyPlayer => PlayerTarget::EnnemyPlayer,
        PlayerTemplateTarget::Player => PlayerTarget::Player,
        PlayerTemplateTarget::BothPlayers => PlayerTarget::BothPlayers,
    }
}

pub fn convert_to_effect(teff: &TemplateEffect, initiator_id: InstanceId) -> Effect {
    match teff {
        TemplateEffect::MakeDraw { player, amount } => Effect::MakeDraw {
            initiator: initiator_id,
            player: convert_template_player_target(player),
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
        TemplateEffect::Summon { side, target } => Effect::Summon {
            initiator: initiator_id,
            side: convert_template_player_target(side),
            target: target.clone(),
        },
    }
}
