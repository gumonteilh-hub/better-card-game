use crate::error::{Error, Result};
use crate::game::action::Action;
use crate::game::effects::Target;
use crate::game::types::InstanceId;

pub fn compute(
    context: &mut crate::Game,
    initiator: &InstanceId,
    target: &Target,
    amount: &usize,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();

    // Heal players
    let player_targets = super::resolve_target_player_only(*initiator, target, context)?;
    for player_id in player_targets {
        let player = context.get_mut_player(player_id)?;
        let max_hp = 30;
        let old_hp = player.hp;
        player.hp = (player.hp + *amount).min(max_hp);
        let effective_heal = player.hp - old_hp;

        if effective_heal > 0 {
            actions.push(Action::Heal {
                target: player_id,
                amount: effective_heal,
            });
        }
    }

    // Heal entities
    let entity_targets = super::resolve_field_target(*initiator, target, context)?;
    for target_id in entity_targets {
        let entity = context.get_mut_entity(target_id)?;
        match &mut entity.card_type {
            crate::game::card::CardTypeInstance::Monster(monster_instance) => {
                let max_hp = monster_instance.max_hp;
                let old_hp = monster_instance.hp;
                monster_instance.hp = (monster_instance.hp + *amount).min(max_hp);
                let effective_heal = monster_instance.hp - old_hp;

                if effective_heal > 0 {
                    actions.push(Action::Heal {
                        target: target_id,
                        amount: effective_heal,
                    });
                }
            }
            crate::game::card::CardTypeInstance::Spell(_) => {
                return Err(Error::Game("Can't heal a spell".into()));
            }
        }
    }

    Ok(actions)
}
