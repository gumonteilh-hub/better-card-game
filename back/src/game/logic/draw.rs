use crate::error::Result;
use crate::game::action::Action;
use crate::game::effects::PlayerTarget;
use crate::game::types::{InstanceId, Location, PlayerId};

pub fn compute_make_draw(
    context: &mut crate::Game,
    initiator: &InstanceId,
    player: &PlayerTarget,
    amount: &usize,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();
    let targets = super::resolve_player_target(*initiator, player, context)?;

    for player_id in targets {
        for _ in 0..*amount {
            if let Some(card_id) = context
                .entities
                .iter()
                .find(|(_, card)| {
                    card.owner == player_id && card.location == Location::Deck
                })
                .map(|(id, _)| *id)
            {
                let hand_full = context.get_hand(player_id).len() >= 10;
                if let Some(card) = context.entities.get_mut(&card_id) {
                    if !hand_full {
                        card.location = Location::Hand;
                        actions.push(Action::Draw {
                            player: player_id,
                            card: card.clone(),
                        });
                        let oponent = context.get_opponent(&player_id)?;
                        actions.push(Action::EnemyDraw {
                            player: oponent.player_id,
                        });
                    } else {
                        card.location = Location::Graveyard;
                        actions.push(Action::BurnCard {
                            player: player_id,
                            card: card.id,
                        });
                    }
                }
            } else {
                // Todo implement fatigue
            }
        }
    }

    Ok(actions)
}

pub fn compute_auto_draw(
    context: &mut crate::Game,
    player: &PlayerId,
    amount: &usize,
) -> Result<Vec<Action>> {
    let mut actions = Vec::new();

    for _ in 0..*amount {
        if let Some(card_id) = context
            .entities
            .iter()
            .find(|(_, card)| card.owner == *player && card.location == Location::Deck)
            .map(|(id, _)| *id)
        {
            let hand_not_full = context.get_hand(*player).len() < 10;
            if let Some(card) = context.entities.get_mut(&card_id) {
                if hand_not_full {
                    card.location = Location::Hand;
                    actions.push(Action::Draw {
                        player: *player,
                        card: card.clone(),
                    });
                    let oponent = context.get_opponent(player)?;
                    actions.push(Action::EnemyDraw {
                        player: oponent.player_id,
                    });
                } else {
                    card.location = Location::Graveyard;
                    actions.push(Action::BurnCard {
                        player: *player,
                        card: card.id,
                    });
                }
            }
        } else {
            // Todo implement fatigue
        }
    }

    Ok(actions)
}
