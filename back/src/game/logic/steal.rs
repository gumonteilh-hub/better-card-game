use crate::error::Result;
use crate::game::action::Action;
use crate::game::types::{InstanceId, Location};
use rand::prelude::*;

pub fn compute(
    context: &mut crate::Game,
    initiator: &InstanceId,
    target_location: &Location,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();

    // Determine thief and victim
    let thief_id = if *initiator < 2 {
        *initiator
    } else {
        context.get_entity(*initiator)?.owner
    };
    let victim_id = super::get_opponent_player_id(thief_id, context)?;

    // Find all cards in the target location belonging to the victim
    let eligible_cards: Vec<InstanceId> = context
        .entities
        .iter()
        .filter(|(_, card)| {
            card.owner == victim_id && match target_location {
                Location::Deck => card.location == Location::Deck,
                Location::Hand => card.location == Location::Hand,
                Location::Graveyard => card.location == Location::Graveyard,
                Location::Field(_) => matches!(card.location, Location::Field(_)),
            }
        })
        .map(|(id, _)| *id)
        .collect();

    // If no cards available, return empty actions
    if eligible_cards.is_empty() {
        return Ok(actions);
    }

    // Select a random card
    let mut rng = rand::rng();
    let selected_card_id = *eligible_cards.choose(&mut rng).unwrap();

    // Check if thief's hand is full
    let hand_full = context.get_hand(thief_id).len() >= 10;

    if let Some(card) = context.entities.get_mut(&selected_card_id) {
        let from_location = card.location.clone();
        let stolen_card = card.clone();

        // Change owner and location
        card.owner = thief_id;

        if hand_full {
            // If hand is full, card goes to thief's graveyard
            card.location = Location::Graveyard;
        } else {
            // Otherwise, card goes to thief's hand
            card.location = Location::Hand;
        }

        // Create the CardStolen action
        actions.push(Action::CardStolen {
            thief: thief_id,
            victim: victim_id,
            card: stolen_card,
            from_location,
        });
    }

    Ok(actions)
}
