use crate::error::{Error, Result};
use crate::game::action::Action;
use crate::game::effects::Target;
use crate::game::types::InstanceId;

pub fn compute(
    context: &mut crate::Game,
    initiator: &InstanceId,
    attack: &usize,
    hp: &usize,
    target: &Target,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();
    let targets = super::resolve_field_target(*initiator, target, context)?;

    for target_id in targets {
        let target = context.get_mut_entity(target_id)?;
        match &mut target.card_type {
            crate::game::card::CardTypeInstance::Monster(monster_instance) => {
                monster_instance.attack += attack;
                monster_instance.hp += hp;
                monster_instance.max_hp += hp;

                actions.push(Action::Boost {
                    target: target_id,
                    attack: *attack,
                    hp: *hp,
                });
            }
            crate::game::card::CardTypeInstance::Spell(_) => {
                return Err(Error::Game("Can't boost a spell".into()));
            }
        }
    }

    Ok(actions)
}
