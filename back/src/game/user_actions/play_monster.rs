use crate::{
    error::{Error, Result},
    game::{
        action::Action,
        card::CardTypeInstance,
        types::{InstanceId, Location, PlayerId},
    },
};

pub fn play_monster(
    context: &mut crate::Game,
    owner: PlayerId,
    card_id: InstanceId,
    position: usize,
    selected_targets: Option<Vec<InstanceId>>,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();
    if context.get_field(owner).len() >= 8 {
        return Err(Error::Game("Your board is already full".into()));
    }

    if context
        .get_field(owner)
        .iter()
        .any(|(_, c)| c.location == Location::Field(position))
    {
        return Err(Error::Game(
            "This place on the field is not empty".to_string(),
        ));
    }

    let card = context.get_entity(card_id)?;

    if !matches!(card.location, Location::Hand) {
        return Err(Error::Game(
            "This card must be in your hand to play it".to_string(),
        ));
    }

    let card_cost = card.cost;
    let player = context
        .players
        .get_mut(&owner)
        .ok_or_else(|| Error::Game(format!("Player with id {} not found", owner)))?;

    if player.mana < card_cost {
        return Err(Error::Game(
            "You don't have enough mana to play this card".into(),
        ));
    }
    player.mana -= card_cost;

    let card = context.get_entity(card_id)?;
    let on_play_effect = if let CardTypeInstance::Monster(monster) = &card.card_type {
        if let Some(target) = card.play_target {
            if let Some(selecteds) = selected_targets {
                validate_target(target, &selecteds, owner, context)?;
                monster
                    .on_play
                    .iter()
                    .map(|effect| crate::game::utils::map_to_choosen_target(effect, &selecteds))
                    .collect()
            } else {
                monster.on_play.clone()
            }
        } else {
            monster.on_play.clone()
        }
    } else {
        return Err(Error::Game(
            "You are trying to play a spell as a monster".into(),
        ));
    };

    let card = context.get_mut_entity(card_id)?;

    actions.push(Action::Summon {
        source: Location::Hand,
        destination: position,
        target: card.clone(),
        owner,
    });

    card.location = Location::Field(position);

    if let CardTypeInstance::Monster(monster) = &mut card.card_type
        && monster
            .keywords
            .contains(&crate::game::card::Keyword::Charge)
    {
        monster.asleep = false;
    }

    if !on_play_effect.is_empty() {
        actions.push(Action::TriggerOnPlay(card_id));
        context.effect_queue.extend(on_play_effect);
    }

    Ok(actions)
}

pub(super) fn validate_target(
    target: crate::collection::types::PlayTarget,
    selecteds: &[InstanceId],
    owner: PlayerId,
    context: &crate::Game,
) -> crate::error::Result<()> {
    if target.strict && selecteds.len() != target.amount {
        return Err(Error::Game(format!(
            "Wrong quantity of targets selected (required: {})",
            &target.amount
        )));
    } else if selecteds.len() > target.amount {
        return Err(Error::Game(format!(
            "Too many targets selected (maximum: {})",
            &target.amount
        )));
    }
    for &select in selecteds.iter() {
        let entity = context.get_entity(select)?;
        if !crate::game::utils::match_entity(owner, entity, target.matcher) {
            return Err(Error::Game(
                "You selected a target that doesn't match the card conditions".to_string(),
            ));
        }
    }
    Ok(())
}
