use crate::{
    error::{Error, Result},
    game::{
        Game,
        action::Action,
        effects::{Effect, PlayerTarget, Target},
        types::{InstanceId, Location, PlayerId},
    },
};

mod summon;

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
                match &mut target.card_type {
                    super::card::CardTypeInstance::Monster(monster_instance) => {
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
                    super::card::CardTypeInstance::Spell(spell_instance) => {
                        return Err(Error::Game("Can't deal damage to a spell".into()));
                    }
                }
            }
        }
        Effect::Destroy { initiator, target } => {
            let targets = resolve_field_target(*initiator, target, context)?;
            for target in targets {
                let target_entity = context.entities.get_mut(&target).ok_or_else(|| {
                    Error::Game(format!("Entity with id {} not found for destroy", target))
                })?;
                target_entity.location = Location::Graveyard;
                match &target_entity.card_type {
                    super::card::CardTypeInstance::Monster(monster_instance) => {
                        if !monster_instance.on_death.is_empty() {
                            actions.push(Action::TriggerOnDeath(target));
                            context
                                .effect_queue
                                .extend(monster_instance.on_death.clone());
                        }
                        actions.push(Action::Destroy { target });
                    }
                    super::card::CardTypeInstance::Spell(spell_instance) => {
                        return Err(Error::Game("Can't destroy a spell".into()));
                    }
                }
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
                match &mut entity.card_type {
                    super::card::CardTypeInstance::Monster(monster_instance) => {
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
                    super::card::CardTypeInstance::Spell(spell_instance) => {
                        return Err(Error::Game("Can't heal a spell".into()));
                    }
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
            match &mut entity.card_type {
                super::card::CardTypeInstance::Monster(monster_instance) => {
                    if !monster_instance.on_play.is_empty() {
                        actions.push(Action::TriggerOnPlay(*entity_id));
                        context
                            .effect_queue
                            .extend(monster_instance.on_play.clone());
                    }
                    if monster_instance
                        .keywords
                        .contains(&super::card::Keyword::Charge)
                    {
                        monster_instance.asleep = false;
                    }
                }
                super::card::CardTypeInstance::Spell(spell_instance) => {
                    return Err(Error::Game("Can't summon a spell".into()));
                }
            }
        }
        Effect::Attack { initiator, target } => {
            let targets = resolve_target(*initiator, target, context)?;
            for target_id in targets {
                let initiator_entity = context.entities.get_mut(initiator).ok_or_else(|| {
                    Error::Game(format!("Entity with id {} not found for attack", initiator))
                })?;
                match &initiator_entity.card_type {
                    super::card::CardTypeInstance::Monster(monster_instance) => {
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
                    super::card::CardTypeInstance::Spell(spell_instance) => {
                        return Err(Error::Game("Can't attack with a spell".into()));
                    }
                }

                if !is_player_id(target_id) {
                    let target_entity = context.get_entity(target_id)?;
                    match &target_entity.card_type {
                        super::card::CardTypeInstance::Monster(monster_instance) => {
                            context.effect_queue.push_back(Effect::DealDamage {
                                initiator: target_id,
                                target: Target::Id(*initiator),
                                amount: monster_instance.attack,
                            });
                        }
                        super::card::CardTypeInstance::Spell(spell_instance) => {
                            return Err(Error::Game("Can't attack a spell".into()));
                        }
                    }
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
                match &mut target.card_type {
                    super::card::CardTypeInstance::Monster(monster_instance) => {
                        monster_instance.attack += attack;
                        monster_instance.hp += hp;
                        monster_instance.max_hp += hp;

                        actions.push(Action::Boost {
                            target: target_id,
                            attack: *attack,
                            hp: *hp,
                        });
                    }
                    super::card::CardTypeInstance::Spell(spell_instance) => {
                        return Err(Error::Game("Can't boost a spell".into()));
                    }
                }
            }
        }
        Effect::Summon {
            initiator,
            side,
            target,
        } => {
            let summon_actions = summon::compute(context, initiator, side, target)?;
            actions.extend(summon_actions);
        }
    }

    Ok(actions)
}

fn is_player_id(id: usize) -> bool {
    id < 2
}

fn resolve_target(
    initiator: InstanceId,
    target: &Target,
    context: &Game,
) -> Result<Vec<InstanceId>> {
    let mut targets = Vec::new();
    let mut player_targets = resolve_target_player_only(initiator, target, context)?;
    targets.append(&mut player_targets);
    let mut field_targets = resolve_field_target(initiator, target, context)?;
    targets.append(&mut field_targets);

    Ok(targets)
}
fn resolve_target_player_only(
    initiator: InstanceId,
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
    initiator: InstanceId,
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
    initiator: InstanceId,
    target: &Target,
    context: &Game,
) -> Result<Vec<InstanceId>> {
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
