use crate::error::{Error, Result};
use crate::game::action::Action;
use crate::game::effects::{Effect, Target};
use crate::game::types::InstanceId;

fn is_player_id(id: usize) -> bool {
    id < 2
}

pub fn compute(
    context: &mut crate::Game,
    initiator: &InstanceId,
    target: &Target,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();
    let targets = super::resolve_target(*initiator, target, context)?;

    for target_id in targets {
        let initiator_entity = context.entities.get_mut(initiator).ok_or_else(|| {
            Error::Game(format!("Entity with id {} not found for attack", initiator))
        })?;
        match &initiator_entity.card_type {
            crate::game::card::CardTypeInstance::Monster(monster_instance) => {
                if !monster_instance.on_attack.is_empty() {
                    actions.push(Action::TriggerOnAttack(initiator_entity.id));
                    context
                        .effect_queue
                        .extend(monster_instance.on_attack.clone());
                }
                context.effect_queue.push_back(Effect::DealDamage {
                    initiator: *initiator,
                    target: Target::Id(target_id),
                    amount: monster_instance.attack,
                });
            }
            crate::game::card::CardTypeInstance::Spell(_) => {
                return Err(Error::Game("Can't attack with a spell".into()));
            }
        }

        if !is_player_id(target_id) {
            let target_entity = context.get_entity(target_id)?;
            match &target_entity.card_type {
                crate::game::card::CardTypeInstance::Monster(monster_instance) => {
                    context.effect_queue.push_back(Effect::DealDamage {
                        initiator: target_id,
                        target: Target::Id(*initiator),
                        amount: monster_instance.attack,
                    });
                }
                crate::game::card::CardTypeInstance::Spell(_) => {
                    return Err(Error::Game("Can't attack a spell".into()));
                }
            }
        }
        actions.push(Action::Attack {
            initiator: *initiator,
            target: target_id,
        });
    }

    Ok(actions)
}
