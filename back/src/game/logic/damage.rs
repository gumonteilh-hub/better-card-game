use crate::error::{Error, Result};
use crate::game::action::Action;
use crate::game::effects::{Effect, Target};
use crate::game::types::InstanceId;

pub fn compute(
    context: &mut crate::Game,
    initiator: &InstanceId,
    target: &Target,
    amount: &usize,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();

    // Deal damage to players
    let player_targets = super::resolve_target_player_only(*initiator, target, context)?;
    for target_id in player_targets {
        let target = context.get_mut_player(target_id)?;
        target.hp = target.hp.saturating_sub(*amount);
        if target.hp == 0 {
            let winner_id = super::get_opponent_player_id(target_id, context)?;
            context.effect_queue.push_back(Effect::Win(winner_id));
        }
        actions.push(Action::ReceiveDamage {
            target: target_id,
            amount: *amount,
        });
    }

    // Deal damage to entities
    let entity_targets = super::resolve_field_target(*initiator, target, context)?;
    for target_id in entity_targets {
        let target = context.get_mut_entity(target_id)?;
        match &mut target.card_type {
            crate::game::card::CardTypeInstance::Monster(monster_instance) => {
                monster_instance.hp = monster_instance.hp.saturating_sub(*amount);
                if monster_instance.hp == 0 {
                    context.effect_queue.push_back(Effect::Destroy {
                        initiator: *initiator,
                        target: Target::Id(target_id),
                    });
                }
                actions.push(Action::ReceiveDamage {
                    target: target_id,
                    amount: *amount,
                });
            }
            crate::game::card::CardTypeInstance::Spell(_) => {
                return Err(Error::Game("Can't deal damage to a spell".into()));
            }
        }
    }

    Ok(actions)
}
