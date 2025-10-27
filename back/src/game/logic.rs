use crate::{
    error::{Error, Result},
    game::{
        Game,
        action::Action,
        effects::{Effect, PlayerTarget, Target},
        types::{EntityId, Location, PlayerId},
    },
    template::convert_to_effect,
};

pub fn execute_effect(effect: &Effect, context: &mut Game) -> Result<Vec<Action>> {
    let mut actions: Vec<Action> = Vec::new();
    match effect {
        Effect::MakeDraw {
            initiator,
            player,
            amount,
        } => {
            let initiator_owner = context.get_entity(*initiator)?.owner;
            let opponent_id = get_opponent_player_id(initiator_owner, context)?;
            let targets = resolve_player_target(*initiator, player, context)?;
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
                        let hand_full = context.get_hand(player_id).len() < 10;
                        if let Some(card) = context.entities.get_mut(&card_id) {
                            if hand_full {
                                card.location = Location::Hand;
                                actions.push(Action::Draw {
                                    player: player_id,
                                    card: card.clone(),
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
        }
        Effect::DealDamage {
            initiator,
            target,
            amount,
        } => {
            let player_targets = resolve_target_player_only(*initiator, target, context)?;
            for target_id in player_targets {
                let target = context.get_mut_player(target_id)?;
                target.hp = target.hp.saturating_sub(*amount);
                if target.hp == 0 {
                    let winner_id = get_opponent_player_id(target_id, context)?;
                    context.effect_queue.push_back(Effect::Win(winner_id));
                }
                actions.push(Action::ReceiveDamage {
                    target: target_id,
                    amount: *amount,
                });
            }

            let entity_targets = resolve_field_target(*initiator, target, context)?;
            for target_id in entity_targets {
                let target = context.get_mut_entity(target_id)?;
                target.hp = target.hp.saturating_sub(*amount);
                if target.hp == 0 {
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
        }
        Effect::Destroy { initiator, target } => {
            let targets = resolve_field_target(*initiator, target, context)?;
            for target in targets {
                let target_entity = context.entities.get_mut(&target).ok_or_else(|| {
                    Error::Game(format!("Entity with id {} not found for destroy", target))
                })?;
                target_entity.location = Location::Graveyard;
                if !target_entity.template.on_death.is_empty() {
                    actions.push(Action::TriggerOnDeath(target));
                    context.effect_queue.extend(
                        target_entity
                            .template
                            .on_death
                            .iter()
                            .map(|teff| convert_to_effect(teff, target_entity)),
                    );
                }
                actions.push(Action::Destroy { target });
            }
        }
        Effect::Heal {
            initiator,
            target,
            amount,
        } => {
            let player_targets = resolve_target_player_only(*initiator, target, context)?;
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

            let entity_targets = resolve_field_target(*initiator, target, context)?;
            for target_id in entity_targets {
                let entity = context.get_mut_entity(target_id)?;
                let max_hp = entity.template.hp;
                let old_hp = entity.hp;
                entity.hp = (entity.hp + *amount).min(max_hp);
                let effective_heal = entity.hp - old_hp;

                if effective_heal > 0 {
                    actions.push(Action::Heal {
                        target: target_id,
                        amount: effective_heal,
                    });
                }
            }
        }
        Effect::SummonFromHand {
            entity_id,
            position,
        } => {
            let entity = context.entities.get_mut(entity_id).ok_or_else(|| {
                Error::Game(format!(
                    "Entity with id {} not found for summon from hand",
                    entity_id
                ))
            })?;

            let base_location = entity.location.clone();

            entity.location = Location::Field(*position);
            actions.push(Action::Summon {
                source: base_location,
                destination: *position,
                target: entity.clone(),
                owner: entity.owner,
            });
            if !entity.template.on_play.is_empty() {
                actions.push(Action::TriggerOnPlay(*entity_id));
                context.effect_queue.extend(
                    entity
                        .template
                        .on_play
                        .iter()
                        .map(|teff| convert_to_effect(teff, entity)),
                );
            }
        }
        Effect::Attack { initiator, target } => {
            let targets = resolve_target(*initiator, target, context)?;
            for target_id in targets {
                let initiator_entity = context.entities.get_mut(initiator).ok_or_else(|| {
                    Error::Game(format!("Entity with id {} not found for attack", initiator))
                })?;
                if !initiator_entity.template.on_attack.is_empty() {
                    actions.push(Action::TriggerOnAttack(initiator_entity.id));
                    context.effect_queue.extend(
                        initiator_entity
                            .template
                            .on_attack
                            .iter()
                            .map(|teff| convert_to_effect(teff, initiator_entity)),
                    );
                }
                context.effect_queue.push_back(Effect::DealDamage {
                    initiator: *initiator,
                    target: Target::Id(target_id),
                    amount: initiator_entity.attack,
                });

                if !is_player_id(target_id) {
                    // attack on a monster
                    let target_entity = context.get_entity(target_id)?;
                    context.effect_queue.push_back(Effect::DealDamage {
                        initiator: target_id,
                        target: Target::Id(*initiator),
                        amount: target_entity.attack,
                    });
                }
                actions.push(Action::Attack {
                    initiator: *initiator,
                    target: target_id,
                });
            }
        }
        Effect::Win(player_id) => {
            context.winner_id = Some(*player_id);
            actions.push(Action::Win(*player_id));
        }
        Effect::AutoDraw { player, amount } => {
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
        }
        Effect::IncreaseMaxMana {
            initiator,
            player,
            amount,
        } => {
            let targets = resolve_player_target(*initiator, player, context)?;
            for target in targets {
                context.get_mut_player(target)?.base_mana += amount;
                actions.push(Action::IncreaseMaxMana {
                    player: target,
                    amount: *amount,
                });
            }
        }
        Effect::RefreshMana {
            initiator,
            player,
            amount,
        } => {
            let targets = resolve_player_target(*initiator, player, context)?;
            for target in targets {
                let player = context.get_mut_player(target)?;
                let effective_refresh;
                if player.mana + amount >= player.base_mana {
                    effective_refresh = player.base_mana - player.mana;
                    player.mana = player.base_mana;
                } else {
                    effective_refresh = *amount;
                    player.mana += amount;
                }

                actions.push(Action::RefreshMana {
                    player: target,
                    amount: effective_refresh,
                });
            }
        }
        Effect::Boost {
            initiator,
            attack,
            hp,
            target,
        } => {
            let targets = resolve_field_target(*initiator, target, context)?;

            for target_id in targets {
                let target = context.get_mut_entity(target_id)?;
                target.attack += attack;
                target.hp += hp;

                actions.push(Action::Boost {
                    target: target_id,
                    attack: *attack,
                    hp: *hp,
                });
            }
        }
    }

    Ok(actions)
}

fn is_player_id(id: usize) -> bool {
    id < 2
}

fn resolve_target(initiator: EntityId, target: &Target, context: &Game) -> Result<Vec<EntityId>> {
    let mut targets = Vec::new();
    let mut player_targets = resolve_target_player_only(initiator, target, context)?;
    targets.append(&mut player_targets);
    let mut field_targets = resolve_field_target(initiator, target, context)?;
    targets.append(&mut field_targets);

    Ok(targets)
}
fn resolve_target_player_only(
    initiator: EntityId,
    target: &Target,
    context: &Game,
) -> Result<Vec<PlayerId>> {
    let player_side = if is_player_id(initiator) {
        initiator
    } else {
        context.get_entity(initiator)?.owner
    };

    let opponent_id = get_opponent_player_id(player_side, context)?;
    let targets = match target {
        Target::Player => vec![player_side],
        Target::EnnemyPlayer => vec![opponent_id],
        Target::BothPlayers | Target::All => vec![player_side, opponent_id],
        Target::Id(id) => {
            if context.players.contains_key(id) {
                vec![*id]
            } else {
                vec![]
            }
        }
        _ => vec![], // Not a player target
    };
    Ok(targets)
}

fn resolve_player_target(
    initiator: EntityId,
    target: &PlayerTarget,
    context: &Game,
) -> Result<Vec<PlayerId>> {
    let player_side = if is_player_id(initiator) {
        initiator
    } else {
        context.get_entity(initiator)?.owner
    };

    let opponent_id = get_opponent_player_id(player_side, context)?;
    let targets = match target {
        PlayerTarget::Player => vec![player_side],
        PlayerTarget::EnnemyPlayer => vec![opponent_id],
        PlayerTarget::BothPlayers => vec![player_side, opponent_id],
        PlayerTarget::Id(id) => {
            if context.players.contains_key(id) {
                vec![*id]
            } else {
                vec![]
            }
        }
    };
    Ok(targets)
}

fn get_opponent_player_id(player_id: PlayerId, context: &Game) -> Result<PlayerId> {
    context
        .players
        .keys()
        .find(|p| **p != player_id)
        .copied()
        .ok_or_else(|| {
            Error::Game(format!(
                "Opponent not found for player with id {}",
                player_id
            ))
        })
}

fn resolve_field_target(
    initiator: EntityId,
    target: &Target,
    context: &Game,
) -> Result<Vec<EntityId>> {
    let player_side = if is_player_id(initiator) {
        initiator
    } else {
        context.get_entity(initiator)?.owner
    };

    let opponent_id = get_opponent_player_id(player_side, context)?;
    let targets = match target {
        Target::ItSelf => vec![initiator],
        Target::Allies => context
            .get_field(player_side)
            .iter()
            .map(|e| e.1.id)
            .collect(),
        Target::Ennemies => context
            .get_field(opponent_id)
            .iter()
            .map(|e| e.1.id)
            .collect(),
        Target::AllMonsters => context
            .get_field(player_side)
            .iter()
            .map(|e| e.1.id)
            .chain(context.get_field(opponent_id).iter().map(|e| e.1.id))
            .collect(),
        Target::All => context
            .entities
            .values()
            .filter(|e| matches!(e.location, Location::Field(_)))
            .map(|e| e.id)
            .collect(),
        Target::Id(id) => {
            if context.entities.contains_key(id) {
                vec![*id]
            } else {
                vec![]
            }
        }
        _ => vec![], // Not an entity target
    };
    Ok(targets)
}
