use crate::{
    collection::types::{
        PlayTarget, PlayTargetTemplate, PlayerTemplateTarget, Side, TargetMatcher,
        TargetMatcherTemplate, TemplateEffect, TemplateTarget,
    },
    game::{
        effects::{Effect, PlayerTarget, Target},
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

impl TemplateEffect {
    pub fn convert(self, initiator_id: InstanceId) -> Effect {
        match self {
            TemplateEffect::MakeDraw { player, amount } => Effect::MakeDraw {
                initiator: initiator_id,
                player: player.convert(),
                amount,
            },
            TemplateEffect::Heal { target, amount } => Effect::Heal {
                initiator: initiator_id,
                target: target.convert(),
                amount,
            },
            TemplateEffect::Destroy { target } => Effect::Destroy {
                initiator: initiator_id,
                target: target.convert(),
            },
            TemplateEffect::DealDamage { target, amount } => Effect::DealDamage {
                initiator: initiator_id,
                target: target.convert(),
                amount,
            },
            TemplateEffect::Attack { target } => Effect::Attack {
                initiator: initiator_id,
                target: target.convert(),
            },
            TemplateEffect::Boost { target, attack, hp } => Effect::Boost {
                initiator: initiator_id,
                target: target.convert(),
                attack,
                hp,
            },
            TemplateEffect::Summon { side, target } => Effect::Summon {
                initiator: initiator_id,
                side: side.convert(),
                target: target.clone(),
            },
            Self::RefreshMana { player, amount } => Effect::RefreshMana {
                initiator: initiator_id,
                player: player.convert(),
                amount,
            },
            Self::IncreaseMaxMana { player, amount } => Effect::IncreaseMaxMana {
                initiator: initiator_id,
                player: player.convert(),
                amount,
            },
            Self::IncreaseAbsoluteMaxMana { player, amount } => Effect::IncreaseAbsoluteMaxMana {
                initiator: initiator_id,
                player: player.convert(),
                amount,
            },
            TemplateEffect::DecreaseCurrentMana { player, amount } => Effect::DecreaseCurrentMana {
                initiator: initiator_id,
                player: player.convert(),
                amount,
            },
        }
    }
}

impl TemplateTarget {
    fn convert(self) -> Target {
        match self {
            TemplateTarget::EnnemyPlayer => Target::EnnemyPlayer,
            TemplateTarget::Player => Target::Player,
            TemplateTarget::BothPlayers => Target::BothPlayers,
            TemplateTarget::ItSelf => Target::ItSelf,
            TemplateTarget::Allies => Target::Allies,
            TemplateTarget::Ennemies => Target::Ennemies,
            TemplateTarget::AllMonsters => Target::AllMonsters,
            TemplateTarget::All => Target::All,
            TemplateTarget::Choose => Target::Ids(vec![]),
            TemplateTarget::Matching(target_matcher) => Target::Matching(target_matcher),
            TemplateTarget::And(a, b) => Target::And(Box::new(a.convert()), Box::new(b.convert())),
            TemplateTarget::Or(a, b) => Target::Or(Box::new(a.convert()), Box::new(b.convert())),
        }
    }
}

impl PlayerTemplateTarget {
    fn convert(self) -> PlayerTarget {
        match self {
            PlayerTemplateTarget::EnnemyPlayer => PlayerTarget::EnnemyPlayer,
            PlayerTemplateTarget::Player => PlayerTarget::Player,
            PlayerTemplateTarget::BothPlayers => PlayerTarget::BothPlayers,
        }
    }
}
