use crate::{
    error::{Error, Result},
    game::card::{self, CardTypeInstance},
};

pub fn play_spell(
    context: &mut crate::game::Game,
    owner: crate::game::types::PlayerId,
    card_id: usize,
    selected_targets: Option<Vec<crate::game::types::InstanceId>>,
) -> Result<()> {
    let card_clone = context.get_entity(card_id)?.clone();
    let card_cost = card_clone.cost;

    if !matches!(card_clone.location, crate::game::types::Location::Hand) {
        return Err(Error::Game(
            "This card must be in your hand to play it".to_string(),
        ));
    }

    let player = context
        .players
        .get(&owner)
        .ok_or_else(|| Error::Game(format!("Player with id {} not found", owner)))?;

    if player.mana < card_cost {
        return Err(Error::Game(
            "You don't have enough mana to play this card".into(),
        ));
    }

    match &card_clone.card_type {
        CardTypeInstance::Spell(spell_instance) => {
            if let Some(target) = card_clone.play_target {
                if let Some(selecteds) = selected_targets {
                    super::play_monster::validate_target(target, &selecteds, owner, context)?;
                    let effects = spell_instance.effect.iter().map(|effect| {
                        crate::game::utils::map_to_choosen_target(effect, &selecteds)
                    });
                    context.effect_queue.extend(effects);
                } else {
                    context.effect_queue.extend(spell_instance.effect.clone());
                }
            } else {
                context.effect_queue.extend(spell_instance.effect.clone());
            }
        }
        card::CardTypeInstance::Monster(monster_instance) => {
            return Err(Error::Game(
                "You can not cast a monster, only a spell".into(),
            ));
        }
    }

    let player = context
        .players
        .get_mut(&owner)
        .ok_or_else(|| Error::Game(format!("Player with id {} not found", owner)))?;

    player.mana -= card_cost;

    context.get_mut_entity(card_id)?.location = crate::game::types::Location::Graveyard;
    Ok(())
}
