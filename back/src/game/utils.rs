use crate::game::types::{InstanceId, PlayerId};

pub fn match_entity(
    owner: PlayerId,
    entity: &super::card::CardInstance,
    matcher: crate::collection::types::TargetMatcher,
) -> bool {
    match matcher {
        crate::collection::types::TargetMatcher::Race(race) => entity.race == race,
        crate::collection::types::TargetMatcher::Class(class) => entity.class == class,
        crate::collection::types::TargetMatcher::Side(side) => match side {
            crate::collection::types::Side::Player => entity.owner == owner,
            crate::collection::types::Side::Enemy => entity.owner != owner,
        },
    }
}

fn replace_target_ids(
    target: &super::effects::Target,
    selecteds: &[InstanceId],
) -> super::effects::Target {
    match target {
        super::effects::Target::Ids(_) => super::effects::Target::Ids(selecteds.to_vec()),
        super::effects::Target::And(a, b) => super::effects::Target::And(
            Box::new(replace_target_ids(a, selecteds)),
            Box::new(replace_target_ids(b, selecteds)),
        ),
        super::effects::Target::Or(a, b) => super::effects::Target::Or(
            Box::new(replace_target_ids(a, selecteds)),
            Box::new(replace_target_ids(b, selecteds)),
        ),
        other => other.clone(),
    }
}

pub(crate) fn map_to_choosen_target(
    effect: &super::effects::Effect,
    selecteds: &Vec<InstanceId>,
) -> super::effects::Effect {
    match effect {
        super::effects::Effect::Heal {
            initiator,
            target,
            amount,
        } => super::effects::Effect::Heal {
            initiator: *initiator,
            target: replace_target_ids(target, selecteds),
            amount: *amount,
        },
        super::effects::Effect::Destroy { initiator, target } => super::effects::Effect::Destroy {
            initiator: *initiator,
            target: replace_target_ids(target, selecteds),
        },
        super::effects::Effect::DealDamage {
            initiator,
            target,
            amount,
        } => super::effects::Effect::DealDamage {
            initiator: *initiator,
            target: replace_target_ids(target, selecteds),
            amount: *amount,
        },
        super::effects::Effect::Attack { initiator, target } => super::effects::Effect::Attack {
            initiator: *initiator,
            target: replace_target_ids(target, selecteds),
        },
        super::effects::Effect::Boost {
            initiator,
            attack,
            hp,
            target,
        } => super::effects::Effect::Boost {
            initiator: *initiator,
            attack: *attack,
            hp: *hp,
            target: replace_target_ids(target, selecteds),
        },
        other => other.clone(),
    }
}
