use crate::{
    Game,
    error::{Error, Result},
    game::{
        card::{CardTypeInstance, Keyword},
        effects::{Effect, Target},
        types::{InstanceId, Location, PlayerId},
    },
};

pub const DEFENSE_POSITIONS: [usize; 5] = [1, 2, 4, 5, 7];
pub const ATTACK_POSITIONS: [usize; 5] = [0, 2, 3, 5, 6];

pub fn attack(
    context: &mut Game,
    player: PlayerId,
    initiator_id: InstanceId,
    target_id: InstanceId,
) -> Result<()> {
    let initiator = context
        .entities
        .get(&initiator_id)
        .ok_or_else(|| Error::Game(format!("Attacker with id {} not found", initiator_id)))?;

    if target_id == 0 || target_id == 1 {
        if initiator.owner == target_id {
            return Err(Error::Game("You can't attack your own player".into()));
        }
        if context
            .get_field_with_position(target_id)
            .iter()
            .any(|(pos, _)| DEFENSE_POSITIONS.contains(pos))
        {
            return Err(Error::Game(
                "You can't attack the enemy player if he has a monster in defense".into(),
            ));
        }
    } else {
        let target = context
            .entities
            .get(&target_id)
            .ok_or_else(|| Error::Game(format!("Target with id {} not found", target_id)))?;

        if initiator.owner == target.owner {
            return Err(Error::Game("You can't attack your own monster".into()));
        }
    }
    match initiator.location {
        Location::Field(pos) => {
            if !ATTACK_POSITIONS.contains(&pos) {
                return Err(Error::Game(
                    "This monster must be on an attack slot to attack".into(),
                ));
            }
        }
        _ => {
            return Err(Error::Game(
                "This monster must be on the field to attack".into(),
            ));
        }
    };
    let initiator = context
        .entities
        .get_mut(&initiator_id)
        .ok_or_else(|| Error::Game(format!("Attacker with id {} not found", initiator_id)))?;

    match &mut initiator.card_type {
        CardTypeInstance::Monster(monster_instance) => {
            if monster_instance.asleep {
                return Err(Error::Game(
                    "This monster can't attack on his first turn".into(),
                ));
            }

            if monster_instance.keywords.contains(&Keyword::Windfury) {
                if monster_instance.attack_count > 1 {
                    return Err(Error::Game(
                        "This monster has already attacked this turn".into(),
                    ));
                }
            } else if monster_instance.attack_count > 0 {
                return Err(Error::Game(
                    "This monster has already attacked this turn".into(),
                ));
            }

            context.effect_queue.push_back(Effect::Attack {
                initiator: initiator_id,
                target: Target::Id(target_id),
            });
            monster_instance.attack_count += 1;

            Ok(())
        }
        CardTypeInstance::Spell(spell_instance) => {
            Err(Error::Game("A spell can not attack".into()))
        }
    }
}
