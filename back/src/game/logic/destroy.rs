use crate::error::{Error, Result};
use crate::game::action::Action;
use crate::game::effects::Target;
use crate::game::types::{InstanceId, Location};

pub fn compute(
    context: &mut crate::Game,
    initiator: &InstanceId,
    target: &Target,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();
    let targets = super::resolve_field_target(*initiator, target, context)?;

    for target in targets {
        let target_entity = context.entities.get_mut(&target).ok_or_else(|| {
            Error::Game(format!("Entity with id {} not found for destroy", target))
        })?;
        target_entity.location = Location::Graveyard;
        match &target_entity.card_type {
            crate::game::card::CardTypeInstance::Monster(monster_instance) => {
                if !monster_instance.on_death.is_empty() {
                    actions.push(Action::TriggerOnDeath(target));
                    context
                        .effect_queue
                        .extend(monster_instance.on_death.clone());
                }
                actions.push(Action::Destroy { target });
            }
            crate::game::card::CardTypeInstance::Spell(_) => {
                return Err(Error::Game("Can't destroy a spell".into()));
            }
        }
    }

    Ok(actions)
}
