use crate::{
    collection::types::{
        PlayTarget, PlayTargetTemplate, Side, TargetMatcher, TargetMatcherTemplate,
    },
    game::{
        effects::Effect,
        types::{InstanceId, PlayerId},
    },
};

impl PlayTargetTemplate {
    pub fn convert(&self, owner: PlayerId, oponent_id: PlayerId) -> PlayTarget {
        PlayTarget {
            strict: self.strict,
            amount: self.amount,
            matcher: self.matcher.convert(owner, oponent_id),
        }
    }
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

impl Effect {
    pub fn convert(self, initiator_id: InstanceId) -> Effect {
        match self {
            Effect::IncreaseMaxMana { player, amount, .. } => Effect::IncreaseMaxMana {
                initiator: initiator_id,
                player,
                amount,
            },
            Effect::DecreaseCurrentMana { player, amount, .. } => Effect::DecreaseCurrentMana {
                initiator: initiator_id,
                player,
                amount,
            },
            Effect::RefreshMana { player, amount, .. } => Effect::RefreshMana {
                initiator: initiator_id,
                player,
                amount,
            },
            Effect::IncreaseAbsoluteMaxMana { player, amount, .. } => {
                Effect::IncreaseAbsoluteMaxMana {
                    initiator: initiator_id,
                    player,
                    amount,
                }
            }
            Effect::MakeDraw { player, amount, .. } => Effect::MakeDraw {
                initiator: initiator_id,
                player,
                amount,
            },
            Effect::AutoDraw { player, amount } => Effect::AutoDraw { player, amount },
            Effect::Heal { target, amount, .. } => Effect::Heal {
                initiator: initiator_id,
                target,
                amount,
            },
            Effect::Destroy { target, .. } => Effect::Destroy {
                initiator: initiator_id,
                target,
            },
            Effect::DealDamage { target, amount, .. } => Effect::DealDamage {
                initiator: initiator_id,
                target,
                amount,
            },
            Effect::Attack { target, .. } => Effect::Attack {
                initiator: initiator_id,
                target,
            },
            Effect::Boost {
                attack, hp, target, ..
            } => Effect::Boost {
                initiator: initiator_id,
                attack,
                hp,
                target,
            },
            Effect::Summon { side, target, .. } => Effect::Summon {
                initiator: initiator_id,
                side,
                target,
            },
            Effect::Win(player) => Effect::Win(player),
        }
    }
}
