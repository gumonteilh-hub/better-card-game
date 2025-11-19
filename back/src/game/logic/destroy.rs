use crate::error::{Error, Result};
use crate::game::action::Action;
use crate::game::card::CardTypeInstance;
use crate::game::effects::Target;
use crate::game::types::{InstanceId, Location};

pub fn compute(
    context: &mut crate::Game,
    initiator_id: &InstanceId,
    target: &Target,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();
    let targets = super::resolve_field_target(*initiator_id, target, context)?;

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
        let initiator = context.get_entity(*initiator_id)?;
        if let CardTypeInstance::Monster(monster) = &initiator.card_type
            && !monster.on_kill.is_empty()
        {
            actions.push(Action::TriggerOnKill(*initiator_id));
            context.effect_queue.extend(monster.on_kill.clone());
        }
    }

    let positional_actions = super::trigger_positional_effects(context)?;
    actions.extend(positional_actions);

    Ok(actions)
}
